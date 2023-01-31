mod background_widget;
mod texture_widget;

pub use background_widget::*;
pub use texture_widget::*;

// Scene counter for the purpose of widget fragment shader
#[derive(Default, Clone, Copy)]
pub struct PrimitiveCounter {
    pub background_num: i32,
    pub texture_num: i32,
    pub border_num: i32,
}

// Enum that translates to numbers, since GLSL doesn't support them
#[derive(Clone, Copy)]
pub enum PrimitiveType {
    Background = 1,
    Texture = 2,
    Border = 3
}