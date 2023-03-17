pub mod widgets;
pub mod window_utils;
pub mod resource_manager;
pub mod error;
pub mod game_object;
pub mod camera;
pub mod render_pipelines;
pub mod scenes;

// TODO: remember to tighten these restrictions up in a way that makes sense
pub use widgets::*;
pub use window_utils::*;
pub use resource_manager::*;
pub use error::*;
pub use game_object::*;
pub use camera::*;
pub use render_pipelines::*;
pub use scenes::*;

// Lib level uses
use std::cell::RefCell;
use silver_gl::ModelTrait;

// TODO: Add type aliases for everything (textures, etc)
// TODO: Use compile-time targets to change the meaning
// TODO: of the type aliases, allowing for A) easier
// TODO: code re-use with different GL libs, and B)
// TODO: the ability to remove things like Boxed traits
// TODO: where they are not needed, optimising the code
// Type aliases to make the multi-GL usage easier
type Model = RefCell<Box<dyn ModelTrait>>;

// TODO: To support multiple OpenGL versions, DX11-12, and Vulkan
// TODO: To do this, move all GL-specific code into their separate
// TODO: GL crates, e.g. silver_gl_4.6, silver_gl_web, silver_vk,
// TODO: etc. All versions of silver_gl will be forks, the rest
// TODO: are separate repositories.
// TODO: Depending on how it's done, there are two options.
// TODO: 1. Compile-time targets that import different crates
// TODO:    with the same forward-facing API
// TODO: 2. Different versions of the engine, that are then exported
// TODO:    in lib with compile time targets.
// TODO: 3. (This is the most likely one) Wrappers for each API that
// TODO:    handle connecting the multi-purpose GL libs to the engine.
// TODO:    This will then expose forward facing API to the rest of
// TODO:    the engine that will be imported based on compile-time
// TODO:    targets. This will be useful if for example, widgets
// TODO:    require drawing individually in OpenGL 3.3 due to lack
// TODO:    of multi-rendering capabilities, making the multi-purpose
// TODO:    libs too generalized.
// TODO: Idea came from here: https://github.com/gfx-rs/gfx/blob/master/src/backend/gl/src/lib.rs