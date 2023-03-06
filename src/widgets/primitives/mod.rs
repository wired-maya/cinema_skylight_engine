mod background_widget;
mod texture_widget;
mod border_widget;
mod text_widget;

pub use background_widget::*;
pub use texture_widget::*;
pub use border_widget::*;
pub use text_widget::*;

// Enum that translates to numbers, since GLSL doesn't support them
#[derive(Clone, Copy)]
pub enum PrimitiveType {
    Background = 1,
    Texture = 2,
    Border = 3,
    Text = 4
}