use std::sync::mpsc::Receiver;
use glfw::{Window, Glfw, WindowEvent, Context};

pub struct GlWindow {
    pub glfw: Glfw,
    pub events: Option<Receiver<(f64, WindowEvent)>>,
    pub window: Window,
}

impl GlWindow {
    pub fn new(width: u32, height: u32, title: &str) -> GlWindow {
        // Create window
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        #[cfg(target_os = "macos")] glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        let (mut window, events) = glfw.create_window(
            width, height, title, glfw::WindowMode::Windowed
        ).expect("Failed to create GLFW window");

        // Apply window options
        window.make_current();
        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_scroll_polling(true);
        window.set_cursor_mode(glfw::CursorMode::Disabled);

        // Move into struct for easy referencing
        GlWindow { glfw, events: Some(events), window }
    }
}