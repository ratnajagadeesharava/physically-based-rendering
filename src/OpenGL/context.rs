use windows::core::Result;

/// Legacy context file - see renderer.rs for new implementation.
/// Kept for backwards compatibility.
pub fn run_opengl() -> Result<()> {
    crate::OpenGL::run()
}
