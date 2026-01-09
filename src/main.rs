//! Computer Graphics 
//! 
//! Usage:
//!   cargo run           # OpenGL  (default)
//!   cargo run -- --dx   # DirectX 12 
//!   cargo run -- --gl   # OpenGL  (explicit)

#![allow(warnings)]

mod DirectX;
mod OpenGL;
mod renderer;
mod util;

use windows::core::Result;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GraphicsAPI {
    OpenGL,
    DirectX,
}

impl Default for GraphicsAPI {
    fn default() -> Self {
        GraphicsAPI::OpenGL  // OpenGL is now the default
    }
}

/// Parse command line arguments to select backend.
fn parse_graphics_api() -> GraphicsAPI {
    let args: Vec<String> = std::env::args().collect();
    
    for arg in &args[1..] {
        match arg.as_str() {
            "--dx" | "--directx" | "--d3d" | "--d3d12" => return GraphicsAPI::DirectX,
            "--gl" | "--opengl" => return GraphicsAPI::OpenGL,
            "--help" | "-h" => {
                println!("Computer Graphics - Multi-backend renderer\n");
                println!("Usage: {} [OPTIONS]\n", args[0]);
                println!("Options:");
                println!("  --gl, --opengl     Use OpenGL backend (default)");
                println!("  --dx, --directx    Use DirectX 12 backend");
                println!("  --help, -h         Show this help message");
                std::process::exit(0);
            }
            _ => {}
        }
    }
    
    GraphicsAPI::default()
}

fn main() -> Result<()> {
    let graphics_api = parse_graphics_api();
    
    match graphics_api {
        GraphicsAPI::OpenGL => {
            println!("=== OpenGL Backend ===\n");
            OpenGL::run()
        },
        GraphicsAPI::DirectX => {
            println!("=== DirectX 12 Backend ===\n");
            DirectX::run()
        }
    }
}
