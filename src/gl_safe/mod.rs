use glfw::Window;
use crate::debug::debug_message_callback;

mod shader_program;
mod error;

pub use shader_program::*;
pub use error::GlError;

pub fn init(window: &mut Window) {
    // Create GL context
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    configure();
}

pub fn configure() {
    unsafe {
        // Enable depth testing
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LESS);

        // Blending
        // gl::Enable(gl::BLEND);
        // gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        gl::Disable(gl::BLEND);

        // Face culling
        gl::Enable(gl::CULL_FACE);

        // Enable debug with callback for simple error printing
        gl::Enable(gl::DEBUG_OUTPUT);
        gl::DebugMessageCallback(
            Some(debug_message_callback),
            std::ptr::null()
        );

        // Enable multisampling
        // gl::Enable(gl::MULTISAMPLE);

        // Draw in wireframe
        // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
    }
}