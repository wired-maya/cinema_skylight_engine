pub mod widgets;
pub mod cs_engine;
pub mod resource_manager;
pub mod error;
pub mod game_object;
pub mod camera;
pub mod render_pipelines;
pub mod scenes;

// TODO: remember to tighten these restrictions up in a way that makes sense
pub use widgets::*;
pub use cs_engine::*;
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

// TODO: add default shaders for all widgets that are available anywhere,
// TODO: and use them to create default for every widget (and remove their new funcs)

// TODO: remember to set all properties to sensible pubs!

// TODO: expand on all primitive widgets (fit style for texture widget, etc)

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
// TODO: Look into how interesting you might be able to do things with no GL selected

// TODO: Text animation
// TODO: Use animated for intro/outro of individual text, allow for each font to have its own as well
// TODO: To that end, support animated textures

// TODO: transform_anchor
// TODO: Select from an enum of the four cardinal directions plus corners and centre, with an offset of relative
// TODO: units (stored within enum) for x and y. This is to allow for animations to focus on different parts
// TODO: when wiping. E.g. a shrinking picture to the right, choose whether the centre shifts to the centre
// TODO: of the new frame, or if the image does not shift at all (right anchor)

// TODO: Clip spaces
// TODO: Each widget has one, is a new struct containing x,y and width,height as well as rotation and radius
// TODO: Size of widgets also uses this struct, so clip spaces are essentially invisible widgets that clip
// TODO: Radius is in relative units horizontal, which will be translated to dots/pixels, radius is for corners
// TODO: Shader handles drawing/discarding, basically if it is outside of clip for the widget it is not drawn
// TODO: Sizes for the clip widget correspond relative to the parent widet, so default is (x: 0, y: 0, width 1.0,
// TODO: height 1.0, etc). Clip space val is an option, so clipping can be off. Clipping is used to hide animations
// TODO: (for example, all textures will be resized once moving animation is done, text wiping up/to the side, etc)
// TODO: Clipping can therefore be turned off for cool effects, or simply debugging.
// TODO: A significantly better way to do it is to have a clip bool that lives on the parent widget as to whether
// TODO: to clip or not. Have this in tandem with a visibility bool that lives on the parent widget as well, which
// TODO: just discards at the top of the frag shader. All of this info is stored in the widget block data.

// TODO: Begging ideas of animation system
// TODO: Some kind of struct or something that's held by either a central animation system, or the widgets themselves
// TODO: Need to support an "animation_done" signal to allow for things like textures resizing/disappearing
// TODO: Need to support individual properties for more advanced effects, for example shrinking clipping after widget
// TODO: Maybe each animation is a struct that has an animate function that takes a widget reference, then changes its
// TODO: properties each frame with a anim_step property (most likely held by the widget itself?)
// TODO: The central animation manager then handles registering which functions get which animations, etc.