use crate::widgets::Drawable;

pub type _Widget = Box<dyn Drawable>;

pub struct _UiState {
    render_stack: Vec<_Widget> // Should be fixed size
}

