use std::sync::mpsc::Receiver;
use glfw::{Window, Glfw, WindowEvent, Context};

pub struct CSEngine {
    pub glfw: Glfw,
    pub events: Receiver<(f64, WindowEvent)>,
    pub window: Window,
}

impl CSEngine {
    pub fn create_window(&mut self) {
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
        CSEngine { glfw, events, window }
    }

    pub fn extension_supported(&self, extension: &str) -> bool {
        self.glfw.extension_supported(extension)
    }
}

pub struct CSEngineConfig {
    pub width: u32,
    pub height: u32,
    pub fov: f32,
    pub title: String,
    pub gl: GraphicsLibrary
}

impl Default for CSEngineConfig {
    fn default() -> Self {
        Self {
            width: 1920,
            height: 1080,
            fov: 45.0,
            title: String::from("My Game"),
            gl: GraphicsLibrary::OpenGL4_6 // TODO: default should be 3_3
        }
    }
}

pub enum GraphicsLibrary {
    OpenGL4_6
}