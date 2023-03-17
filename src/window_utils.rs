use std::sync::mpsc::Receiver;
use glfw::{Window, Glfw, WindowEvent, Context};

pub struct EngineWindow {
    pub glfw: Glfw,
    pub events: Receiver<(f64, WindowEvent)>,
    pub window: Window,
}

impl EngineWindow {
    pub fn new(window_config: WindowConfig) -> EngineWindow {
        // Create window
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(4, 6));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        #[cfg(target_os = "macos")] glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        let (mut window, events) = glfw.create_window(
            window_config.width,
            window_config.height,
            window_config.title.as_str(),
            glfw::WindowMode::Windowed
        ).expect("Failed to create GLFW window");

        // Apply window options
        window.make_current();
        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_scroll_polling(true);
        // TODO: Temporary
        // window.set_cursor_mode(glfw::CursorMode::Disabled);

        // Move into struct for easy referencing
        EngineWindow { glfw, events, window }
    }

    pub fn extension_supported(&self, extension: &str) -> bool {
        self.glfw.extension_supported(extension)
    }
}

pub struct WindowConfig {
    pub width: u32,
    pub height: u32,
    pub title: String
}