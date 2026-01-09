//! Renderer abstraction trait.
//! Both DirectX and OpenGL backends implement this trait.

use windows::core::Result;

/// Common renderer trait for graphics backends.
pub trait Renderer {
    /// Initialize the graphics context (device, swap chain, pipeline, etc.)
    fn init(&mut self) -> Result<()>;
    
    /// Render a single frame.
    fn render(&mut self) -> Result<()>;
    
    /// Run the main loop (message pump / event loop).
    fn run(&mut self) -> Result<()>;
}

/// Configuration for window and renderer.
#[derive(Debug, Clone)]
pub struct RendererConfig {
    pub width: u32,
    pub height: u32,
    pub title: &'static str,
    pub frame_count: u32,
}

impl Default for RendererConfig {
    fn default() -> Self {
        Self {
            width: 1920,
            height: 1080,
            title: "Computer Graphics",
            frame_count: 2,
        }
    }
}

/// Vertex data shared across backends.
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Vertex {
    pub pos: [f32; 4],
}

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { pos: [x, y, z, w] }
    }
}

/// Triangle vertices (shared demo geometry).
pub fn triangle_vertices() -> [Vertex; 3] {
    [
        Vertex::new(0.0, 0.5, 0.0, 1.0),
        Vertex::new(0.5, -0.5, 0.0, 1.0),
        Vertex::new(-0.5, -0.5, 0.0, 1.0),
    ]
}
