use std::{sync::mpsc::Receiver, ffi::{c_void, CString}, slice};
use glfw::{Window, Glfw, WindowEvent, Context};
use silver_gl::gl;
use crate::ResourceManager;

pub struct CSEngine {
    pub glfw: Glfw,
    pub events: Receiver<(f64, WindowEvent)>,
    pub window: Window,
    pub resource_manager: ResourceManager,
    config: CSEngineConfig,
}

impl CSEngine {
    pub fn new(config: CSEngineConfig) -> Self {
        let (glfw, window, events) = Self::create_window(
            config.width as u32,
            config.height as u32,
            &config.title,
            config.capture_mouse,
            config.gl
        );

        let mut engine = CSEngine {
            glfw,
            events,
            window,
            resource_manager: ResourceManager::new(),
            config
        };

        // Probe extension support
        match engine.config.gl {
            GraphicsLibrary::OpenGL4_6(_, ref mut exts) => {
                // exts.supports_bindless = engine.extension_supported("GL_ARB_bindless_texture");
                exts.supports_bindless = false; // TODO: temp to get this working
            },
            GraphicsLibrary::None => {},
        }

        engine.configure_gl();
        engine.resource_manager.gl = engine.config.gl; // Set here so RM can react to changes in GL settings

        engine
    }

    pub fn create_window(
        width: u32,
        height: u32,
        title: &str,
        capture_mouse: bool,
        gl: GraphicsLibrary
    ) -> (Glfw, Window, Receiver<(f64, WindowEvent)>) {
        // Create window
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        match gl {
            GraphicsLibrary::OpenGL4_6(_, _) => {
                glfw.window_hint(glfw::WindowHint::ContextVersion(4, 6));
                glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
                #[cfg(target_os = "macos")] glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
            },
            GraphicsLibrary::None => {},
        }

        let (mut window, events) = glfw.create_window(
            width,
            height,
            title,
            glfw::WindowMode::Windowed // TODO: add to engine config
        ).expect("Failed to create GLFW window");

        // Apply window options
        window.make_current();
        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_scroll_polling(true);
        
        if capture_mouse {
            window.set_cursor_mode(glfw::CursorMode::Disabled);
        }

        // Move into struct for easy referencing
        (glfw, window, events)
    }

    pub fn configure_gl(&mut self) {
        match self.config.gl {
            GraphicsLibrary::OpenGL4_6(_, _) => unsafe {
                // Create GL context
                gl::load_with(|symbol| self.window.get_proc_address(symbol) as *const _);
        
                // Depth testing
                gl::Enable(gl::DEPTH_TEST);
                gl::DepthFunc(gl::LESS);
        
                // Blending
                // gl::Enable(gl::BLEND);
                // gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
                gl::Disable(gl::BLEND);
        
                // Face culling
                gl::Enable(gl::CULL_FACE);
        
                // Enable debug with callback for simple error printing
                match self.config.debug_level {
                    DebugLevel::High => {
                        gl::Enable(gl::DEBUG_OUTPUT);
                        gl::DebugMessageCallback(
                            Some(Self::debug_message_callback_high),
                            std::ptr::null()
                        );
                    },
                }
            },
            GraphicsLibrary::None => {},
        }
    }

    pub fn extension_supported(&self, extension: &str) -> bool {
        self.glfw.extension_supported(extension)
    }

    // Callback function intended to be called from C
    extern "system" fn debug_message_callback_high(
        source: u32,
        type_: u32,
        _id: u32,
        severity: u32,
        length: i32,
        message: *const i8,
        _user_param: *mut c_void
    ) {
        let type_str = match type_ {
            gl::DEBUG_TYPE_ERROR => "ERROR",
            gl::DEBUG_TYPE_DEPRECATED_BEHAVIOR => "DEPRECATED_BEHAVIOR",
            gl::DEBUG_TYPE_UNDEFINED_BEHAVIOR => "UNDEFINED_BEHAVIOR",
            gl::DEBUG_TYPE_PORTABILITY => "PORTABILITY",
            gl::DEBUG_TYPE_PERFORMANCE => "PERFORMANCE",
            gl::DEBUG_TYPE_MARKER => "MARKER",
            gl::DEBUG_TYPE_PUSH_GROUP => "PUSH_GROUP",
            gl::DEBUG_TYPE_POP_GROUP => "POP_GROUP",
            gl::DEBUG_TYPE_OTHER => "OTHER",
            _ => "OTHER"
        };
        let source_str = match source {
            gl::DEBUG_SOURCE_API => "API",
            gl::DEBUG_SOURCE_WINDOW_SYSTEM => "WINDOW_SYSTEM",
            gl::DEBUG_SOURCE_SHADER_COMPILER => "SHADER_COMPILER",
            gl::DEBUG_SOURCE_THIRD_PARTY => "THIRD_PARTY",
            gl::DEBUG_SOURCE_APPLICATION => "APPLICATION",
            gl::DEBUG_SOURCE_OTHER => "OTHER",
            _ => "OTHER"
        };
        let severity_str = match severity {
            gl::DEBUG_SEVERITY_HIGH => "HIGH_SEVERITY",
            gl::DEBUG_SEVERITY_MEDIUM => "MEDIUM_SEVERITY",
            gl::DEBUG_SEVERITY_LOW => "LOW_SEVERITY",
            gl::DEBUG_SEVERITY_NOTIFICATION => "NOTIFICATION",
            _ => "UNKNOWN_SEVERITY"
        };
        let message_cstr = unsafe {
            let buffer = slice::from_raw_parts(message as *const u8, length as usize);
            CString::from_vec_unchecked(buffer.to_vec())
        };
    
        println!("{}::{}::{}::{}", type_str, source_str, severity_str, message_cstr.to_str().unwrap());
    }
}

#[derive(Debug, Clone)]
pub struct CSEngineConfig {
    pub width: i32,
    pub height: i32,
    pub fov: f32,
    pub title: String,
    pub gl: GraphicsLibrary,
    pub capture_mouse: bool,
    pub debug_level: DebugLevel
}

impl Default for CSEngineConfig {
    fn default() -> Self {
        Self {
            width: 1920,
            height: 1080,
            fov: 45.0,
            title: String::from("My Game"),
            gl: GraphicsLibrary::OpenGL4_6(Default::default(), Default::default()), // TODO: default should be 3_3
            capture_mouse: true,
            debug_level: DebugLevel::High // TODO: change to Medium
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum GraphicsLibrary {
    OpenGL4_6(OpenGL4_6Config, OpenGLExtSupport),
    None
}

#[derive(Debug, Clone, Copy)]
pub struct OpenGL4_6Config { }

impl Default for OpenGL4_6Config {
    fn default() -> Self {
        Self { }
    }
}

// Struct that contains information on extension support which the engine might need
// so you don't need to keep probing the GL context
#[derive(Debug, Clone, Copy)]
pub struct OpenGLExtSupport {
    pub supports_bindless: bool
}

impl Default for OpenGLExtSupport {
    fn default() -> Self {
        Self {
            supports_bindless: false
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum DebugLevel {
    High
}