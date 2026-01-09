//! DirectX 12 Renderer implementation.

use crate::renderer::{Renderer, RendererConfig, Vertex, triangle_vertices};
use core::ffi::c_void;
use std::ffi::OsString;
use std::fs::File;
use std::io::prelude::*;
use std::mem::ManuallyDrop;
use std::os::windows::ffi::OsStringExt;
use std::path::PathBuf;

use windows::{
    Win32::{
        Foundation::*,
        Graphics::{
            Direct3D::Fxc::*, Direct3D::*, Direct3D12::*, Dxgi::Common::*, Dxgi::*,
            Gdi::ValidateRect,
        },
        System::LibraryLoader::GetModuleHandleA,
        UI::WindowsAndMessaging::*,
    },
    core::*,
};

/// DirectX 12 renderer state.
pub struct DX12Renderer {
    pub config: RendererConfig,
    // Core objects
    factory: Option<IDXGIFactory6>,
    device: Option<ID3D12Device1>,
    command_queue: Option<ID3D12CommandQueue>,
    swap_chain: Option<IDXGISwapChain3>,
    command_allocator: Option<ID3D12CommandAllocator>,
    command_list: Option<ID3D12GraphicsCommandList>,
    // Render targets
    rtv_heap: Option<ID3D12DescriptorHeap>,
    rtv_descriptor_size: u32,
    back_buffers: Vec<Option<ID3D12Resource>>,
    frame_index: u32,
    // Pipeline
    root_signature: Option<ID3D12RootSignature>,
    pipeline_state: Option<ID3D12PipelineState>,
    // Geometry
    vertex_buffer: Option<ID3D12Resource>,
    vb_view: Option<D3D12_VERTEX_BUFFER_VIEW>,
    // Window
    hwnd: Option<HWND>,
}

impl DX12Renderer {
    pub fn new(config: RendererConfig) -> Self {
        Self {
            config,
            factory: None,
            device: None,
            command_queue: None,
            swap_chain: None,
            command_allocator: None,
            command_list: None,
            rtv_heap: None,
            rtv_descriptor_size: 0,
            back_buffers: Vec::new(),
            frame_index: 0,
            root_signature: None,
            pipeline_state: None,
            vertex_buffer: None,
            vb_view: None,
            hwnd: None,
        }
    }

    /// Convert DXGI description to string.
    fn dxgi_desc_to_string(desc: &[u16]) -> String {
        let len = desc.iter().position(|&c| c == 0).unwrap_or(desc.len());
        OsString::from_wide(&desc[..len])
            .to_string_lossy()
            .into_owned()
    }

    /// Read shader file from disk.
    fn read_shader_file(path: &str) -> Result<String> {
        let mut abs_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        abs_path.push(path);
        println!("Loading shader: {:?}", abs_path);
        let mut file = File::open(abs_path).expect("unable to open shader file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("unable to read shader file");
        Ok(contents)
    }

    /// Create DXGI factory and find adapter.
    unsafe fn create_factory_and_device(&mut self) -> Result<()> {
        let factory: IDXGIFactory6 = CreateDXGIFactory1()?;
        
        let mut i: u32 = 0;
        let mut adapter: Option<IDXGIAdapter1> = None;
        while let Ok(adptr) = factory.EnumAdapters1(i) {
            let desc: DXGI_ADAPTER_DESC1 = adptr.GetDesc1()?;
            let flag = desc.Flags as i32;
            if flag != DXGI_ERROR_NOT_FOUND.0 {
                adapter = Some(adptr);
                println!("Found GPU: {:?}", Self::dxgi_desc_to_string(&desc.Description));
                break;
            }
            i += 1;
        }

        let adapter = adapter.expect("No suitable GPU adapter found");
        let mut device: Option<ID3D12Device1> = None;
        D3D12CreateDevice(&adapter, D3D_FEATURE_LEVEL_11_0, &mut device)?;
        
        self.factory = Some(factory);
        self.device = device;
        Ok(())
    }

    /// Create command queue.
    unsafe fn create_command_queue(&mut self) -> Result<()> {
        let device = self.device.as_ref().unwrap();
        let mut desc = D3D12_COMMAND_QUEUE_DESC::default();
        desc.Type = D3D12_COMMAND_LIST_TYPE_DIRECT;
        desc.Flags = D3D12_COMMAND_QUEUE_FLAG_NONE;
        self.command_queue = Some(device.CreateCommandQueue(&desc)?);
        Ok(())
    }

    /// Create Win32 window.
    unsafe fn create_window(&mut self) -> Result<()> {
        let module: HMODULE = GetModuleHandleA(None)?;
        let instance = HINSTANCE(module.0);

        let wc = WNDCLASSA {
            lpfnWndProc: Some(Self::window_proc),
            hInstance: instance,
            lpszClassName: s!("DX12Window"),
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            style: CS_HREDRAW | CS_VREDRAW,
            ..Default::default()
        };
        
        let atom = RegisterClassA(&wc);
        debug_assert!(atom != 0);

        let hwnd = CreateWindowExA(
            WINDOW_EX_STYLE::default(),
            s!("DX12Window"),
            PCSTR::from_raw(format!("{}\0", self.config.title).as_ptr()),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            self.config.width as i32,
            self.config.height as i32,
            None,
            None,
            Some(instance),
            None,
        )?;

        self.hwnd = Some(hwnd);
        Ok(())
    }

    /// Create swap chain.
    unsafe fn create_swap_chain(&mut self) -> Result<()> {
        let factory = self.factory.as_ref().unwrap();
        let queue = self.command_queue.as_ref().unwrap();
        let hwnd = self.hwnd.unwrap();

        let desc = DXGI_SWAP_CHAIN_DESC1 {
            BufferCount: self.config.frame_count,
            Width: self.config.width,
            Height: self.config.height,
            Format: DXGI_FORMAT_R8G8B8A8_UNORM,
            BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
            SwapEffect: DXGI_SWAP_EFFECT_FLIP_DISCARD,
            SampleDesc: DXGI_SAMPLE_DESC { Count: 1, ..Default::default() },
            ..Default::default()
        };

        let swap_chain1: IDXGISwapChain1 = factory.CreateSwapChainForHwnd(queue, hwnd, &desc, None, None)?;
        let swap_chain: IDXGISwapChain3 = swap_chain1.cast()?;
        self.frame_index = swap_chain.GetCurrentBackBufferIndex();
        self.swap_chain = Some(swap_chain);
        Ok(())
    }

    /// Create render target views.
    unsafe fn create_render_targets(&mut self) -> Result<()> {
        let device = self.device.as_ref().unwrap();
        let swap_chain = self.swap_chain.as_ref().unwrap();

        let rtv_desc = D3D12_DESCRIPTOR_HEAP_DESC {
            NumDescriptors: self.config.frame_count,
            Type: D3D12_DESCRIPTOR_HEAP_TYPE_RTV,
            ..Default::default()
        };
        
        let rtv_heap: ID3D12DescriptorHeap = device.CreateDescriptorHeap(&rtv_desc)?;
        let mut rtv_handle = rtv_heap.GetCPUDescriptorHandleForHeapStart();
        let rtv_size = device.GetDescriptorHandleIncrementSize(D3D12_DESCRIPTOR_HEAP_TYPE_RTV);

        let mut back_buffers: Vec<Option<ID3D12Resource>> = vec![None; self.config.frame_count as usize];
        
        for i in 0..self.config.frame_count {
            back_buffers[i as usize] = Some(swap_chain.GetBuffer(i)?);
            device.CreateRenderTargetView(back_buffers[i as usize].as_ref(), None, rtv_handle);
            rtv_handle.ptr += rtv_size as usize;
        }

        self.rtv_heap = Some(rtv_heap);
        self.rtv_descriptor_size = rtv_size;
        self.back_buffers = back_buffers;
        Ok(())
    }

    /// Create command allocator and list.
    unsafe fn create_command_list(&mut self) -> Result<()> {
        let device = self.device.as_ref().unwrap();
        
        let allocator: ID3D12CommandAllocator = device.CreateCommandAllocator(D3D12_COMMAND_LIST_TYPE_DIRECT)?;
        let command_list: ID3D12GraphicsCommandList = device.CreateCommandList(0, D3D12_COMMAND_LIST_TYPE_DIRECT, &allocator, None)?;
        command_list.Close()?;

        self.command_allocator = Some(allocator);
        self.command_list = Some(command_list);
        Ok(())
    }

    /// Create root signature.
    unsafe fn create_root_signature(&mut self) -> Result<()> {
        let device = self.device.as_ref().unwrap();

        let mut desc = D3D12_ROOT_SIGNATURE_DESC::default();
        desc.Flags = D3D12_ROOT_SIGNATURE_FLAG_ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT;

        let mut blob: Option<ID3DBlob> = None;
        D3D12SerializeRootSignature(&desc, D3D_ROOT_SIGNATURE_VERSION_1, &mut blob, None);
        
        let blob = blob.unwrap();
        let data = std::slice::from_raw_parts(
            blob.GetBufferPointer() as *const u8,
            blob.GetBufferSize(),
        );
        
        self.root_signature = Some(device.CreateRootSignature(0, data)?);
        Ok(())
    }

    /// Compile shader from source.
    unsafe fn compile_shader(source: &str, entry: PCSTR, target: PCSTR) -> Result<ID3DBlob> {
        let mut blob: Option<ID3DBlob> = None;
        let mut error: Option<ID3DBlob> = None;

        let result = D3DCompile(
            source.as_ptr() as *const c_void,
            source.len(),
            None,
            None,
            None,
            entry,
            target,
            0,
            0,
            &mut blob,
            Some(&mut error),
        );

        if result.is_err() {
            if let Some(error_blob) = error {
                let error_msg = std::slice::from_raw_parts(
                    error_blob.GetBufferPointer() as *const u8,
                    error_blob.GetBufferSize(),
                );
                let error_str = std::str::from_utf8_unchecked(error_msg);
                panic!("Shader compilation failed:\n{}", error_str);
            } else {
                panic!("Shader compilation failed with no error message");
            }
        }

        Ok(blob.unwrap())
    }

    /// Create graphics pipeline state.
    unsafe fn create_pipeline_state(&mut self) -> Result<()> {
        let device = self.device.as_ref().unwrap();
        let root_signature = self.root_signature.as_ref().unwrap();

        let vs_source = Self::read_shader_file("src\\shaders\\triangle\\vertex.hlsl")?;
        let ps_source = Self::read_shader_file("src\\shaders\\triangle\\pixel.hlsl")?;

        let vs_blob = Self::compile_shader(&vs_source, s!("main"), s!("vs_5_0"))?;
        let ps_blob = Self::compile_shader(&ps_source, s!("main"), s!("ps_5_0"))?;

        let input_layout = vec![D3D12_INPUT_ELEMENT_DESC {
            SemanticName: s!("POSITION"),
            SemanticIndex: 0,
            Format: DXGI_FORMAT_R32G32B32A32_FLOAT,
            InputSlot: 0,
            AlignedByteOffset: 0,
            InputSlotClass: D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA,
            InstanceDataStepRate: 0,
        }];

        let rasterizer_desc = D3D12_RASTERIZER_DESC {
            FillMode: D3D12_FILL_MODE_SOLID,
            CullMode: D3D12_CULL_MODE_NONE,
            ..Default::default()
        };

        let blend_desc = D3D12_RENDER_TARGET_BLEND_DESC {
            RenderTargetWriteMask: D3D12_COLOR_WRITE_ENABLE_ALL.0 as u8,
            ..Default::default()
        };

        let pso_desc = D3D12_GRAPHICS_PIPELINE_STATE_DESC {
            InputLayout: D3D12_INPUT_LAYOUT_DESC {
                pInputElementDescs: input_layout.as_ptr(),
                NumElements: input_layout.len() as u32,
                ..Default::default()
            },
            pRootSignature: ManuallyDrop::new(Some(root_signature.clone())),
            VS: D3D12_SHADER_BYTECODE {
                pShaderBytecode: vs_blob.GetBufferPointer(),
                BytecodeLength: vs_blob.GetBufferSize(),
            },
            PS: D3D12_SHADER_BYTECODE {
                pShaderBytecode: ps_blob.GetBufferPointer(),
                BytecodeLength: ps_blob.GetBufferSize(),
            },
            PrimitiveTopologyType: D3D12_PRIMITIVE_TOPOLOGY_TYPE_TRIANGLE,
            NumRenderTargets: 1,
            SampleDesc: DXGI_SAMPLE_DESC { Count: 1, ..Default::default() },
            SampleMask: u32::MAX,
            BlendState: D3D12_BLEND_DESC {
                RenderTarget: [blend_desc; 8],
                ..Default::default()
            },
            RasterizerState: rasterizer_desc,
            ..Default::default()
        };

        self.pipeline_state = Some(device.CreateGraphicsPipelineState(&pso_desc)?);
        Ok(())
    }

    /// Create vertex buffer with triangle data.
    unsafe fn create_vertex_buffer(&mut self) -> Result<()> {
        let device = self.device.as_ref().unwrap();
        let verts = triangle_vertices();
        let vb_size = std::mem::size_of_val(&verts) as u64;

        let heap_props = D3D12_HEAP_PROPERTIES {
            Type: D3D12_HEAP_TYPE_UPLOAD,
            CPUPageProperty: D3D12_CPU_PAGE_PROPERTY_UNKNOWN,
            MemoryPoolPreference: D3D12_MEMORY_POOL_UNKNOWN,
            CreationNodeMask: 1,
            VisibleNodeMask: 1,
        };

        let resource_desc = D3D12_RESOURCE_DESC {
            Dimension: D3D12_RESOURCE_DIMENSION_BUFFER,
            Alignment: 0,
            Width: vb_size,
            Height: 1,
            DepthOrArraySize: 1,
            MipLevels: 1,
            Format: DXGI_FORMAT_UNKNOWN,
            SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
            Layout: D3D12_TEXTURE_LAYOUT_ROW_MAJOR,
            Flags: D3D12_RESOURCE_FLAG_NONE,
        };

        let mut vertex_buffer: Option<ID3D12Resource> = None;
        device.CreateCommittedResource(
            &heap_props,
            D3D12_HEAP_FLAG_NONE,
            &resource_desc,
            D3D12_RESOURCE_STATE_GENERIC_READ,
            None,
            &mut vertex_buffer,
        )?;

        let vertex_buffer = vertex_buffer.unwrap();

        // Map and copy vertex data
        let mut mapped: *mut c_void = std::ptr::null_mut();
        vertex_buffer.Map(0, None, Some(&mut mapped))?;
        std::ptr::copy_nonoverlapping(verts.as_ptr() as *const c_void, mapped, vb_size as usize);
        vertex_buffer.Unmap(0, None);

        let vb_view = D3D12_VERTEX_BUFFER_VIEW {
            BufferLocation: vertex_buffer.GetGPUVirtualAddress(),
            SizeInBytes: vb_size as u32,
            StrideInBytes: std::mem::size_of::<Vertex>() as u32,
        };

        self.vertex_buffer = Some(vertex_buffer);
        self.vb_view = Some(vb_view);
        Ok(())
    }

    /// Window procedure callback.
    extern "system" fn window_proc(hwnd: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        unsafe {
            match message {
                WM_PAINT => {
                    ValidateRect(Some(hwnd), None);
                    LRESULT(0)
                }
                WM_DESTROY => {
                    println!("Window destroyed");
                    PostQuitMessage(0);
                    LRESULT(0)
                }
                _ => DefWindowProcA(hwnd, message, wparam, lparam),
            }
        }
    }
}

impl Renderer for DX12Renderer {
    fn init(&mut self) -> Result<()> {
        unsafe {
            self.create_factory_and_device()?;
            self.create_command_queue()?;
            self.create_window()?;
            self.create_swap_chain()?;
            self.create_render_targets()?;
            self.create_command_list()?;
            self.create_root_signature()?;
            self.create_pipeline_state()?;
            self.create_vertex_buffer()?;
            println!("DirectX 12 initialized successfully");
            Ok(())
        }
    }

    fn render(&mut self) -> Result<()> {
        unsafe {
            let allocator = self.command_allocator.as_ref().unwrap();
            let command_list = self.command_list.as_ref().unwrap();
            let pipeline_state = self.pipeline_state.as_ref().unwrap();
            let root_signature = self.root_signature.as_ref().unwrap();
            let swap_chain = self.swap_chain.as_ref().unwrap();
            let rtv_heap = self.rtv_heap.as_ref().unwrap();
            let vb_view = self.vb_view.as_ref().unwrap();

            allocator.Reset()?;
            command_list.Reset(allocator, Some(pipeline_state))?;
            command_list.SetGraphicsRootSignature(root_signature);
            command_list.SetPipelineState(pipeline_state);

            let viewport = D3D12_VIEWPORT {
                TopLeftX: 0.0,
                TopLeftY: 0.0,
                Width: self.config.width as f32,
                Height: self.config.height as f32,
                MinDepth: 0.0,
                MaxDepth: 1.0,
            };

            let scissor = RECT {
                left: 0,
                top: 0,
                right: self.config.width as i32,
                bottom: self.config.height as i32,
            };

            command_list.RSSetViewports(&[viewport]);
            command_list.RSSetScissorRects(&[scissor]);

            // Transition to render target
            let barrier = D3D12_RESOURCE_BARRIER {
                Type: D3D12_RESOURCE_BARRIER_TYPE_TRANSITION,
                Flags: D3D12_RESOURCE_BARRIER_FLAG_NONE,
                Anonymous: D3D12_RESOURCE_BARRIER_0 {
                    Transition: ManuallyDrop::new(D3D12_RESOURCE_TRANSITION_BARRIER {
                        pResource: ManuallyDrop::new(self.back_buffers[self.frame_index as usize].clone()),
                        StateBefore: D3D12_RESOURCE_STATE_PRESENT,
                        StateAfter: D3D12_RESOURCE_STATE_RENDER_TARGET,
                        Subresource: D3D12_RESOURCE_BARRIER_ALL_SUBRESOURCES,
                    }),
                },
            };
            command_list.ResourceBarrier(&[barrier]);

            let rtv_handle = rtv_heap.GetCPUDescriptorHandleForHeapStart();
            command_list.OMSetRenderTargets(1, Some(&rtv_handle), false, None);

            let clear_color = [0.1, 0.1, 0.3, 1.0];
            command_list.ClearRenderTargetView(rtv_handle, &clear_color, Some(&[]));

            command_list.IASetPrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
            command_list.IASetVertexBuffers(0, Some(&[*vb_view]));
            command_list.DrawInstanced(3, 1, 0, 0);

            // Transition back to present
            let barrier_back = D3D12_RESOURCE_BARRIER {
                Type: D3D12_RESOURCE_BARRIER_TYPE_TRANSITION,
                Flags: D3D12_RESOURCE_BARRIER_FLAG_NONE,
                Anonymous: D3D12_RESOURCE_BARRIER_0 {
                    Transition: ManuallyDrop::new(D3D12_RESOURCE_TRANSITION_BARRIER {
                        pResource: ManuallyDrop::new(self.back_buffers[self.frame_index as usize].clone()),
                        StateBefore: D3D12_RESOURCE_STATE_RENDER_TARGET,
                        StateAfter: D3D12_RESOURCE_STATE_PRESENT,
                        Subresource: D3D12_RESOURCE_BARRIER_ALL_SUBRESOURCES,
                    }),
                },
            };
            command_list.ResourceBarrier(&[barrier_back]);

            command_list.Close()?;

            let queue = self.command_queue.as_ref().unwrap();
            queue.ExecuteCommandLists(&[Some(command_list.cast()?)]);

            swap_chain.Present(1, DXGI_PRESENT(0)).ok();
            self.frame_index = swap_chain.GetCurrentBackBufferIndex();

            Ok(())
        }
    }

    fn run(&mut self) -> Result<()> {
        unsafe {
            // Initial render
            self.render()?;

            // Message loop
            let mut msg = MSG::default();
            while GetMessageA(&mut msg, None, 0, 0).into() {
                DispatchMessageA(&msg);
            }
            Ok(())
        }
    }
}

/// Entry point for DirectX backend.
pub fn run() -> Result<()> {
    let config = RendererConfig::default();
    let mut renderer = DX12Renderer::new(config);
    renderer.init()?;
    renderer.run()
}
