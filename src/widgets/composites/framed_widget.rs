use cgmath::{Vector4, Matrix4, vec3};
use crate::{primitives::{BackgroundWidget, BorderWidget}, Widget, EngineError};

// TODO: add from funcs
pub trait FramedWidget: Widget {
    type Inner: Widget; // Type of the widget to be held inside the frame

    // Default order is BackgroundWidget, Inner Widget, BorderWidget,
    // if you change it when implementing a new framed widget then you
    // must reimplement these members
    fn get_background(&self) -> &BackgroundWidget {
        self.get_children()[0].downcast_ref().expect("0th child should be BackgroundWidget!")
    }
    fn get_inner_widget(&self) -> &Self::Inner {
        self.get_children()[1].downcast_ref().expect("1st child should be Self::Inner!")
    }
    fn get_border(&self) -> &BorderWidget {
        self.get_children()[2].downcast_ref().expect("2nd child should be BorderWidget!")
    }

    fn get_background_mut(&mut self) -> &mut BackgroundWidget {
        self.get_children_mut()[0].downcast_mut().expect("0th child should be BackgroundWidget!")
    }
    fn get_inner_widget_mut(&mut self) -> &mut Self::Inner {
        self.get_children_mut()[1].downcast_mut().expect("1st child should be Self::Inner!")
    }
    fn get_border_mut(&mut self) -> &mut BorderWidget {
        self.get_children_mut()[2].downcast_mut().expect("2nd child should be BorderWidget!")
    }

    fn set_background(&mut self, widget: BackgroundWidget) { self.get_children_mut()[0] = Box::new(widget); }
    fn set_inner_widget<T: Widget>(&mut self, widget: T) { self.get_children_mut()[1] = Box::new(widget); }
    fn set_border(&mut self, widget: BorderWidget) { self.get_children_mut()[2] = Box::new(widget); }

    fn get_border_widths(&self) -> &Vector4<f32> {&self.get_border().border_widths }
    fn set_border_widths(&mut self, widths: Vector4<f32>) { self.get_border_mut().border_widths = widths; }

    fn get_padding(&self) -> &Vector4<f32>;
    fn set_padding(&mut self, widths: Vector4<f32>);

    // Reduces vec space of inner widget by padding and borders
    fn draw_children(&mut self, vec_space: &Matrix4<f32>) -> Result<(), EngineError> {
        let padding = self.get_padding();
        let border_widths = self.get_border_widths();

        let width = 1.0 - border_widths.x - border_widths.y - padding.x - padding.y;
        let height = 1.0 - border_widths.z - border_widths.w - padding.z - padding.w;

        let x = border_widths.x + padding.x;
        let y = border_widths.z + padding.z;

        let mut inner_vec_space = Matrix4::<f32>::from_translation(vec3(x, y, 0.0));
        inner_vec_space = inner_vec_space * Matrix4::<f32>::from_nonuniform_scale(width, height, 1.0);
        inner_vec_space = vec_space * inner_vec_space;

        let children = self.get_children_mut();

        children[0].draw(vec_space)?;
        children[1].draw(&inner_vec_space)?;
        children[2].draw(vec_space)?;

        Ok(())
    }
}