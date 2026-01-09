//! OpenGL Renderer implementation with full triangle rendering.

use crate::renderer::{Renderer, RendererConfig, Vertex, triangle_vertices};
use std::ffi::{CStr, CString};
use std::num::NonZeroU32;
use std::ptr;

use glutin::config::ConfigTemplateBuilder;
use glutin::context::{ContextApi, ContextAttributesBuilder, PossiblyCurrentContext, Version};
use glutin::display::GetGlDisplay;
use glutin::prelude::*;
use glutin::surface::{Surface, SurfaceAttributesBuilder, WindowSurface};
use glutin_winit::{DisplayBuilder, GlWindow};
use raw_window_handle::HasRawWindowHandle;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

use windows::core::Result;

/// OpenGL renderer state.
pub struct GLRenderer {
    pub config: RendererConfig,
    // OpenGL objects
    vao: u32,
    vbo: u32,
    shader_program: u32,
    // Context and window
    gl_context: Option<PossiblyCurrentContext>,
    gl_surface: Option<Surface<WindowSurface>>,
    window: Option<Window>,
}

impl GLRenderer {
    pub fn new(config: RendererConfig) -> Self {
        Self {
            config,
            vao: 0,
            vbo: 0,
            shader_program: 0,
            gl_context: None,
            gl_surface: None,
            window: None,
        }
    }

    /// Compile a shader from source.
    unsafe fn compile_shader(source: &str, shader_type: u32) -> u32 {
        let shader = gl::CreateShader(shader_type);
        let c_str = CString::new(source.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        // Check for errors
        let mut success = gl::FALSE as i32;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as i32 {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buffer = vec![0u8; len as usize];
            gl::GetShaderInfoLog(shader, len, ptr::null_mut(), buffer.as_mut_ptr() as *mut i8);
            panic!(
                "Shader compilation failed: {}",
                std::str::from_utf8(&buffer).unwrap()
            );
        }
        shader
    }

    /// Create shader program from vertex and fragment shaders.
    unsafe fn create_shader_program(vs_source: &str, fs_source: &str) -> u32 {
        let vs = Self::compile_shader(vs_source, gl::VERTEX_SHADER);
        let fs = Self::compile_shader(fs_source, gl::FRAGMENT_SHADER);

        let program = gl::CreateProgram();
        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        gl::LinkProgram(program);

        // Check for errors
        let mut success = gl::FALSE as i32;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
        if success != gl::TRUE as i32 {
            let mut len = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buffer = vec![0u8; len as usize];
            gl::GetProgramInfoLog(program, len, ptr::null_mut(), buffer.as_mut_ptr() as *mut i8);
            panic!(
                "Shader linking failed: {}",
                std::str::from_utf8(&buffer).unwrap()
            );
        }

        gl::DeleteShader(vs);
        gl::DeleteShader(fs);
        program
    }

    /// Setup VAO and VBO with triangle vertices.
    unsafe fn setup_vertex_data(&mut self) {
        let vertices = triangle_vertices();
        
        // Create VAO
        gl::GenVertexArrays(1, &mut self.vao);
        gl::BindVertexArray(self.vao);

        // Create VBO
        gl::GenBuffers(1, &mut self.vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<Vertex>()) as isize,
            vertices.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        // Setup vertex attributes (position: vec4)
        gl::VertexAttribPointer(
            0,
            4,
            gl::FLOAT,
            gl::FALSE,
            std::mem::size_of::<Vertex>() as i32,
            ptr::null(),
        );
        gl::EnableVertexAttribArray(0);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }
}

// Vertex shader source (GLSL)
const VERTEX_SHADER_SOURCE: &str = r#"
#version 330 core
layout (location = 0) in vec4 aPos;
void main() {
    gl_Position = aPos;
}
"#;

// Fragment shader source (GLSL)
const FRAGMENT_SHADER_SOURCE: &str = r#"
#version 330 core
out vec4 FragColor;
void main() {
    FragColor = vec4(1.0, 0.0, 0.0, 1.0);
}
"#;

/// Entry point for OpenGL backend.
pub fn run() -> Result<()> {
    let config = RendererConfig::default();
    
    // Create event loop
    let event_loop = EventLoop::new();
    
    // Window builder
    let window_builder = WindowBuilder::new()
        .with_title(config.title)
        .with_inner_size(LogicalSize::new(config.width, config.height));
    
    // Template for OpenGL config - pass the builder, not built template
    let template_builder = ConfigTemplateBuilder::new();
    
    // Use glutin-winit to create display and window together
    let display_builder = DisplayBuilder::new().with_window_builder(Some(window_builder));
    
    let (window, gl_config) = display_builder
        .build(&event_loop, template_builder, |configs| {
            configs.reduce(|accum, config| {
                if config.num_samples() > accum.num_samples() {
                    config
                } else {
                    accum
                }
            }).unwrap()
        })
        .expect("Failed to create display");
    
    let window = window.expect("Failed to create window");
    let gl_display = gl_config.display();
    
    // Get raw window handle for context creation
    let raw_window_handle = window.raw_window_handle();
    
    // Create GL context
    let context_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::OpenGl(Some(Version::new(3, 3))))
        .build(Some(raw_window_handle));
    
    let gl_context = unsafe {
        gl_display
            .create_context(&gl_config, &context_attributes)
            .expect("Failed to create GL context")
    };
    
    // Create GL surface using GlWindow trait
    let attrs = window.build_surface_attributes(SurfaceAttributesBuilder::default());
    let gl_surface = unsafe {
        gl_display
            .create_window_surface(&gl_config, &attrs)
            .expect("Failed to create GL surface")
    };
    
    // Make context current
    let gl_context = gl_context
        .make_current(&gl_surface)
        .expect("Failed to make GL context current");
    
    // Load GL functions
    gl::load_with(|s| gl_display.get_proc_address(&CString::new(s).unwrap()) as *const _);
    
    // Print GL info
    unsafe {
        let version = CStr::from_ptr(gl::GetString(gl::VERSION) as *const i8);
        println!("OpenGL Version: {}", version.to_str().unwrap());
    }
    
    // Create renderer and setup
    let mut renderer = GLRenderer::new(config);
    renderer.gl_context = Some(gl_context);
    renderer.gl_surface = Some(gl_surface);
    renderer.window = Some(window);
    
    unsafe {
        renderer.shader_program = GLRenderer::create_shader_program(VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE);
        renderer.setup_vertex_data();
    }
    
    println!("OpenGL initialized successfully");
    
    // Event loop (winit 0.28 style)
    event_loop.run(move |event, _, control_flow| {
        *control_flow = winit::event_loop::ControlFlow::Wait;
        
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    println!("Window closed");
                    *control_flow = winit::event_loop::ControlFlow::Exit;
                }
                WindowEvent::Resized(size) => {
                    if size.width != 0 && size.height != 0 {
                        if let Some(surface) = &renderer.gl_surface {
                            surface.resize(
                                renderer.gl_context.as_ref().unwrap(),
                                NonZeroU32::new(size.width).unwrap(),
                                NonZeroU32::new(size.height).unwrap(),
                            );
                        }
                        unsafe {
                            gl::Viewport(0, 0, size.width as i32, size.height as i32);
                        }
                    }
                }
                _ => {}
            },
            Event::MainEventsCleared => {
                if let Some(window) = &renderer.window {
                    window.request_redraw();
                }
            }
            Event::RedrawRequested(_) => {
                unsafe {
                    // Clear screen with dark blue
                    gl::ClearColor(0.1, 0.1, 0.3, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                    
                    // Draw triangle
                    gl::UseProgram(renderer.shader_program);
                    gl::BindVertexArray(renderer.vao);
                    gl::DrawArrays(gl::TRIANGLES, 0, 3);
                }
                
                if let Some(surface) = &renderer.gl_surface {
                    if let Some(context) = &renderer.gl_context {
                        surface.swap_buffers(context).expect("Failed to swap buffers");
                    }
                }
            }
            _ => {}
        }
    });
}

impl Renderer for GLRenderer {
    fn init(&mut self) -> Result<()> {
        Ok(())
    }

    fn render(&mut self) -> Result<()> {
        Ok(())
    }

    fn run(&mut self) -> Result<()> {
        Ok(())
    }
}
