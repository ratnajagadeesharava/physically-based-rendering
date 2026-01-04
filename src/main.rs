#![allow(warnings)]
mod DirectX;
mod util;
use crate::util::vector::Vector3;
use core::ffi::c_void;
use std::ffi::OsString;
use std::mem::ManuallyDrop;
use std::os::windows::ffi::OsStringExt;
use std::u32;
use std::{backtrace::BacktraceStatus::Disabled, fs::File, io::prelude::*, path::PathBuf};

use crate::DirectX::{
    context::Context,
    helpers::{Vertex, throw_if_failed},
};

use windows::{
    Win32::{
        Foundation::*,
        Graphics::{
            Direct3D::Fxc::*, Direct3D::*, Direct3D12::*, Dxgi::Common::*, Dxgi::*,
            Gdi::ValidateRect,
        },
        System::{LibraryLoader::GetModuleHandleA, Threading::*, WindowsProgramming::*},
        UI::WindowsAndMessaging::*,
    },
    core::*,
};
const FRAME_COUNT: u32 = 2;
const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;
const WINDOW_NAME: &str = "Computer Graphics";
// https://samrambles.com/guides/window-hacking-with-rust/creating-a-window-with-rust/index.html

fn dxgi_desc_to_string(desc: &[u16]) -> String {
    let len = desc.iter().position(|&c| c == 0).unwrap_or(desc.len());
    OsString::from_wide(&desc[..len])
        .to_string_lossy()
        .into_owned()
}
fn list_adapters() -> Result<()> {
    //factory is an object whose job is to only discover and enumerate the gpu available
    unsafe {
        let factory: IDXGIFactory6 = CreateDXGIFactory1()?;

        let mut i: u32 = 0;
        while let Ok(adapter) = factory.EnumAdapters1(i) {
            let desc: DXGI_ADAPTER_DESC1 = adapter.GetDesc1()?;
            println!("{:?}", dxgi_desc_to_string(&desc.Description));
            println!("{:?}", desc.Flags);
            i = i + 1;
        }
        //Modern GPUs expose multiple nodes internally.
        // DXGI may show:
        // one adapter per node
        // They all report the same name: “NVIDIA GeForce RTX 4090”
        Ok(())
    }
}
fn create_command_queue(device: &ID3D12Device1) -> ID3D12CommandQueue {
    unsafe {
        let mut command_queue_desc = D3D12_COMMAND_QUEUE_DESC::default();
        command_queue_desc.Type = D3D12_COMMAND_LIST_TYPE_DIRECT;
        command_queue_desc.Flags = D3D12_COMMAND_QUEUE_FLAG_NONE;
        let queue = device.CreateCommandQueue(&mut command_queue_desc);
        queue.unwrap()
    }
}
fn create_swap_chain(
    factory: &IDXGIFactory6,
    device: &ID3D12CommandQueue,
    hwnd: &HWND,
) -> Result<IDXGISwapChain1> {
    let swap_chain_desc: DXGI_SWAP_CHAIN_DESC1 = DXGI_SWAP_CHAIN_DESC1 {
        BufferCount: FRAME_COUNT,
        Width: WIDTH,
        Height: HEIGHT,
        Format: DXGI_FORMAT_R8G8B8A8_UNORM,
        BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
        SwapEffect: DXGI_SWAP_EFFECT_FLIP_DISCARD,
        SampleDesc: DXGI_SAMPLE_DESC {
            Count: 1,
            ..Default::default()
        },
        ..DXGI_SWAP_CHAIN_DESC1::default()
    };
    unsafe { factory.CreateSwapChainForHwnd(device, *hwnd, &swap_chain_desc, None, None) }
}
fn create_device(adapter: &IDXGIAdapter1) -> ID3D12Device1 {
    let mut device: Option<ID3D12Device1> = None;
    unsafe { D3D12CreateDevice(adapter, D3D_FEATURE_LEVEL_11_0, &mut device) }
        .expect("TODO: panic message");
    device.unwrap()
}

fn d3d_init(factory: &IDXGIFactory6) -> Result<ID3D12Device1> {
    unsafe {
        let mut i: u32 = 0;
        let mut adapter: Option<IDXGIAdapter1> = None;
        while let Ok(adptr) = factory.EnumAdapters1(i) {
            let desc: DXGI_ADAPTER_DESC1 = adptr.GetDesc1()?;
            let flag = desc.Flags as i32;
            if flag != DXGI_ERROR_NOT_FOUND.0 {
                adapter = Some(adptr);
                println!(" Found {:?}", dxgi_desc_to_string(&desc.Description));
                break;
            }
            i = i + 1;
        }

        Ok(create_device(&adapter.unwrap()))
    }
}

fn create_command_allocator(device: &ID3D12Device1) -> Result<ID3D12CommandAllocator> {
    unsafe {
        let mut allocator: ID3D12CommandAllocator =
            device.CreateCommandAllocator(D3D12_COMMAND_LIST_TYPE_DIRECT)?;
        Ok(allocator)
    }
}
fn create_command_list(
    device: &ID3D12Device1,
    allocator: &ID3D12CommandAllocator,
) -> Result<ID3D12GraphicsCommandList> {
    unsafe {
        let mut command_list: ID3D12GraphicsCommandList =
            device.CreateCommandList(0, D3D12_COMMAND_LIST_TYPE_DIRECT, allocator, None)?;
        Ok(command_list)
    }
}
fn read_file(path: &str) -> Result<String> {
    let mut abs_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    abs_path.push(path);
    println!("{:?}", abs_path);
    let mut file = File::open(abs_path).expect("unable to open file at {path}");
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .expect("unable to read file at {path}");
    Ok(file_contents)
}
fn main() -> Result<()> {
    //win 32 access raw pointers
    // Rust cant verify safety

    // list_adapters()?;

    unsafe {
        let factory: IDXGIFactory6 = CreateDXGIFactory1()?;
        let device = d3d_init(&factory)?;
        let allocator = create_command_allocator(&device)?;
        let command_list = create_command_list(&device, &allocator)?;
        command_list.Close()?;
        //HMODULE == HINSTANCE
        let module: HMODULE = GetModuleHandleA(None)?;
        let instance = HINSTANCE(module.0);

        //properties
        let wc: WNDCLASSA = WNDCLASSA {
            lpfnWndProc: Some(window_proc),
            hInstance: instance,
            lpszClassName: s!("Window"),
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            style: CS_HREDRAW | CS_VREDRAW,
            ..Default::default()
        };
        let atom = RegisterClassA(&wc);
        debug_assert!(atom != 0);
        let hwnd = CreateWindowExA(
            WINDOW_EX_STYLE::default(),
            s!("window"),
            s!("Computer Graphics"),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            None,
            None,
            Some(instance),
            None,
        )?;
        let mut cmd_queue = create_command_queue(&device);
        let swap_chain1: IDXGISwapChain1 = create_swap_chain(&factory, &cmd_queue, &hwnd)?;
        let swap_chain: IDXGISwapChain3 = swap_chain1.cast()?;
        let mut frame_index: u32 = swap_chain.GetCurrentBackBufferIndex();
        let mut rtvDesc: D3D12_DESCRIPTOR_HEAP_DESC = D3D12_DESCRIPTOR_HEAP_DESC::default();
        rtvDesc.NumDescriptors = FRAME_COUNT;
        rtvDesc.Type = D3D12_DESCRIPTOR_HEAP_TYPE_RTV;
        let rtvHeap: ID3D12DescriptorHeap = device.CreateDescriptorHeap(&rtvDesc)?;
        let mut rtvHandle = rtvHeap.GetCPUDescriptorHandleForHeapStart();
        let mut msg: MSG = MSG::default();
        let mut back_buffers: Vec<Option<ID3D12Resource>> = vec![None; FRAME_COUNT as usize];
        let rtv_size = device.GetDescriptorHandleIncrementSize(D3D12_DESCRIPTOR_HEAP_TYPE_RTV);
        //connect the swap chain buffers to render target views Descriptors
        //todo: understand why this forloop works
        for i in 0..FRAME_COUNT {
            let i = i as usize;
            back_buffers[i] = Some(swap_chain.GetBuffer(i as u32)?);
            device.CreateRenderTargetView(back_buffers[i].as_ref(), None, rtvHandle);
            rtvHandle.ptr += rtv_size as usize;
        }

        let mut root_signature_description: D3D12_ROOT_SIGNATURE_DESC =
            D3D12_ROOT_SIGNATURE_DESC::default();
        root_signature_description.Flags =
            D3D12_ROOT_SIGNATURE_FLAG_ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT;

        let mut rsBlob: Option<ID3DBlob> = None;
        D3D12SerializeRootSignature(
            &root_signature_description,
            D3D_ROOT_SIGNATURE_VERSION_1,
            &mut rsBlob,
            None,
        );
        let rs_blob = rsBlob.unwrap();
        let data = std::slice::from_raw_parts(
            rs_blob.GetBufferPointer() as *const u8,
            rs_blob.GetBufferSize(),
        );
        let mut root_signature: ID3D12RootSignature;
        unsafe {
            root_signature = device.CreateRootSignature(0, data)?;
        }

        let v_shader = read_file("src\\shaders\\triangle\\vertex.hlsl")?;
        let p_shader = read_file("src\\shaders\\triangle\\pixel.hlsl")?;
        let mut vs_blob: Option<ID3DBlob> = None;
        let mut ps_blob: Option<ID3DBlob> = None;
        let mut vs_error: Option<ID3DBlob> = None;
        let mut ps_error: Option<ID3DBlob> = None;
        
        let vs_result = D3DCompile(
            v_shader.as_ptr() as *const c_void,
            v_shader.len(),
            None,
            None,
            None,
            s!("main"),
            s!("vs_5_0"),
            0,
            0,
            &mut vs_blob,
            Some(&mut vs_error),
        );
        if vs_result.is_err() {
            if let Some(error_blob) = vs_error {
                let error_msg = std::slice::from_raw_parts(
                    error_blob.GetBufferPointer() as *const u8,
                    error_blob.GetBufferSize(),
                );
                let error_str = std::str::from_utf8_unchecked(error_msg);
                panic!("Vertex shader compilation failed:\n{}", error_str);
            } else {
                panic!("Vertex shader compilation failed with no error message");
            }
        }
        
        let ps_result = D3DCompile(
            p_shader.as_ptr() as *const c_void,
            p_shader.len(),
            None,
            None,
            None,
            s!("main"),
            s!("ps_5_0"),
            0,
            0,
            &mut ps_blob,
            Some(&mut ps_error),
        );
        if ps_result.is_err() {
            if let Some(error_blob) = ps_error {
                let error_msg = std::slice::from_raw_parts(
                    error_blob.GetBufferPointer() as *const u8,
                    error_blob.GetBufferSize(),
                );
                let error_str = std::str::from_utf8_unchecked(error_msg);
                panic!("Pixel shader compilation failed:\n{}", error_str);
            } else {
                panic!("Pixel shader compilation failed with no error message");
            }
        }
        let mut input: Vec<D3D12_INPUT_ELEMENT_DESC> = vec![D3D12_INPUT_ELEMENT_DESC {
            SemanticName: s!("POSITION"),
            SemanticIndex: 0,
            Format: DXGI_FORMAT_R32G32B32A32_FLOAT,
            InputSlot: 0,
            AlignedByteOffset: 0,
            InputSlotClass: D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA,
            InstanceDataStepRate: 0,
        }];
          let mut overlay_rasterize_state = D3D12_RASTERIZER_DESC::default();
        overlay_rasterize_state = D3D12_RASTERIZER_DESC
        {
            FillMode : D3D12_FILL_MODE_SOLID,
            CullMode : D3D12_CULL_MODE_NONE,
            ..overlay_rasterize_state
        };
         let render_target_blend_desc = D3D12_RENDER_TARGET_BLEND_DESC
        {
            RenderTargetWriteMask : D3D12_COLOR_WRITE_ENABLE_ALL.0 as u8,
            ..D3D12_RENDER_TARGET_BLEND_DESC::default()
        };
        let mut rtv_format_list = [DXGI_FORMAT_UNKNOWN; 8];
        rtv_format_list[0] =DXGI_FORMAT_R8G8B8A8_UNORM;

        let mut pipeline_state_desc = D3D12_GRAPHICS_PIPELINE_STATE_DESC {
            InputLayout: D3D12_INPUT_LAYOUT_DESC {
                pInputElementDescs: input.as_ptr(),
                NumElements: input.len() as u32,
                ..Default::default()
            },
            pRootSignature: ManuallyDrop::new(Some(root_signature.clone())),
            VS: D3D12_SHADER_BYTECODE {
                    pShaderBytecode: vs_blob.clone().unwrap().GetBufferPointer(),
                    BytecodeLength: vs_blob.clone().unwrap().GetBufferSize(),
            },
            PS: D3D12_SHADER_BYTECODE {
                    pShaderBytecode: ps_blob.clone().unwrap().GetBufferPointer(),
                    BytecodeLength: ps_blob.clone().unwrap().GetBufferSize(),
            },
            PrimitiveTopologyType: D3D12_PRIMITIVE_TOPOLOGY_TYPE_TRIANGLE,
            NumRenderTargets: 1,
            SampleDesc: DXGI_SAMPLE_DESC {
                Count: 1,
                ..Default::default()
            },
            SampleMask: u32::MAX,
            BlendState:  D3D12_BLEND_DESC
            {
                RenderTarget : [render_target_blend_desc; 8],
                ..D3D12_BLEND_DESC::default()
            },
            RasterizerState: overlay_rasterize_state,
        
            ..Default::default()
        };
        let mut pipeline_state: Option<ID3D12PipelineState> = None;
        if let Ok(x) = device.CreateGraphicsPipelineState(&pipeline_state_desc) {
            pipeline_state = Some(x);
        } else {
            panic!("\npipeline state creation failed !!!!!!!!!!!!!!!!!!\n");
        }

        // TODO: learn about the conversion
        let verts: [Vertex; 3] = [
            Vertex {
                pos: [0.0, 0.5, 0.0, 1.0],
            },
            Vertex {
                pos: [0.5, -0.5, 0.0, 1.0],
            },
            Vertex {
                pos: [-0.5, -0.5, 0.0, 1.0],
            },
        ];
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
            SampleDesc: DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },
            Layout: D3D12_TEXTURE_LAYOUT_ROW_MAJOR,
            Flags: D3D12_RESOURCE_FLAG_NONE,
        };

        let mut vertex_buffer: Option<ID3D12Resource> = None;

        unsafe {
            device.CreateCommittedResource(
                &heap_props,
                D3D12_HEAP_FLAG_NONE,
                &resource_desc,
                D3D12_RESOURCE_STATE_GENERIC_READ,
                None,
                &mut vertex_buffer,
            )?;
        }
        let vertex_buffer = vertex_buffer.unwrap();
        unsafe {
            let mut mapped: *mut c_void = std::ptr::null_mut();

            vertex_buffer.Map(0, None, Some(&mut mapped))?;

            std::ptr::copy_nonoverlapping(
                verts.as_ptr() as *const c_void,
                mapped,
                vb_size as usize,
            );

            vertex_buffer.Unmap(0, None);
        }
        let vb_view = D3D12_VERTEX_BUFFER_VIEW {
            BufferLocation: vertex_buffer.GetGPUVirtualAddress(),
            SizeInBytes: vb_size as u32,
            StrideInBytes: std::mem::size_of::<Vertex>() as u32,
        };
        allocator.Reset();
        command_list.Reset(&allocator, pipeline_state.as_ref().unwrap());
        command_list.SetGraphicsRootSignature(&root_signature);
        command_list.SetPipelineState(pipeline_state.as_ref().unwrap());
        let viewport = D3D12_VIEWPORT {
            TopLeftX: 0.0,
            TopLeftY: 0.0,
            Width: WIDTH as f32,
            Height: HEIGHT as f32,
            MinDepth: 0.0,
            MaxDepth: 1.0,
        };

        let scissor = RECT {
            left: 0,
            top: 0,
            right: WIDTH as i32,
            bottom: HEIGHT as i32,
        };

        command_list.RSSetViewports(&[viewport]);
        command_list.RSSetScissorRects(&[scissor]);
        //TODO: know about this rtv
        let barrier = D3D12_RESOURCE_BARRIER {
            Type: D3D12_RESOURCE_BARRIER_TYPE_TRANSITION,
            Flags: D3D12_RESOURCE_BARRIER_FLAG_NONE,
            Anonymous: D3D12_RESOURCE_BARRIER_0 {
                Transition: ManuallyDrop::new(D3D12_RESOURCE_TRANSITION_BARRIER {
                    pResource: ManuallyDrop::new(Some(
                        back_buffers[frame_index as usize].clone().unwrap(),
                    )),
                    StateBefore: D3D12_RESOURCE_STATE_PRESENT,
                    StateAfter: D3D12_RESOURCE_STATE_RENDER_TARGET,
                    Subresource: D3D12_RESOURCE_BARRIER_ALL_SUBRESOURCES,
                }),
            },
        };

        command_list.ResourceBarrier(&[barrier]);
        let rtv_handle = rtvHeap.GetCPUDescriptorHandleForHeapStart();
        command_list.OMSetRenderTargets(1, Some(&rtv_handle), false, None);
        let clear_color = [0.1, 0.1, 0.3, 1.0];
        command_list.ClearRenderTargetView(rtv_handle, &clear_color, Some(&[]));
        command_list.IASetPrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
        command_list.IASetVertexBuffers(0, Some(&[vb_view]));
        command_list.DrawInstanced(3, 1, 0, 0);

        let barrier_back = D3D12_RESOURCE_BARRIER {
            Type: D3D12_RESOURCE_BARRIER_TYPE_TRANSITION,
            Flags: D3D12_RESOURCE_BARRIER_FLAG_NONE,
            Anonymous: D3D12_RESOURCE_BARRIER_0 {
                Transition: ManuallyDrop::new(D3D12_RESOURCE_TRANSITION_BARRIER {
                    pResource: ManuallyDrop::new(Some(
                        back_buffers[frame_index as usize].clone().unwrap(),
                    )),
                    StateBefore: D3D12_RESOURCE_STATE_RENDER_TARGET,
                    StateAfter: D3D12_RESOURCE_STATE_PRESENT,
                    Subresource: D3D12_RESOURCE_BARRIER_ALL_SUBRESOURCES,
                }),
            },
        };

        command_list.ResourceBarrier(&[barrier_back]);
        command_list.Close()?;

        cmd_queue.ExecuteCommandLists(&[Some(command_list.cast()?)]);
        swap_chain
            .Present(1, windows::Win32::Graphics::Dxgi::DXGI_PRESENT(0))
            .ok();

        frame_index = swap_chain.GetCurrentBackBufferIndex();

        while GetMessageA(&mut msg, None, 0, 0).into() {
            DispatchMessageA(&msg);
        }
        Ok(())
    }
}

extern "system" fn window_proc(
    hwnd: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    unsafe {
        match message {
            WM_PAINT => {
                println!("WM_PAINT");
                ValidateRect(Some(hwnd), None);
                LRESULT(0)
            }
            WM_DESTROY => {
                println!("Destroying window  ");
                PostQuitMessage(0);
                LRESULT(0)
            }
            _ => DefWindowProcA(hwnd, message, wparam, lparam),
        }
    }
}
