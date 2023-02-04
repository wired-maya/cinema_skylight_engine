use cgmath::{Vector4, vec2};

use crate::{primitives::{BackgroundWidget, BorderWidget}, Widget};

pub trait FramedWidget: Widget {
    // Default order is BackgroundWidget, Inner Widget, BorderWidget,
    // if you change it when implementing a new framed widget then you
    // must reimplement these members
    fn get_background(&self) -> &BackgroundWidget {
        self.get_children()[0].downcast_ref().expect("0th child should be BackgroundWidget!")
    }
    // TODO: Use the type variable thing to make return type the actual widget
    // TODO: Use that to also add a from_primitives function
    fn get_inner_widget(&self) -> &Box<dyn Widget> { &self.get_children()[1] }
    fn get_border(&self) -> &BorderWidget {
        self.get_children()[2].downcast_ref().expect("2nd child should be BorderWidget!")
    }

    fn get_background_mut(&mut self) -> &mut BackgroundWidget {
        self.get_children_mut()[0].downcast_mut().expect("0th child should be BackgroundWidget!")
    }
    fn get_inner_widget_mut(&mut self) -> &mut Box<dyn Widget> { &mut self.get_children_mut()[1] }
    fn get_border_mut(&mut self) -> &mut BorderWidget {
        self.get_children_mut()[2].downcast_mut().expect("2nd child should be BorderWidget!")
    }

    fn set_background(&mut self, widget: BackgroundWidget) { self.get_children_mut()[0] = Box::new(widget); }
    fn set_inner_widget<T: Widget>(&mut self, widget: T) { self.get_children_mut()[1] = Box::new(widget); }
    fn set_border(&mut self, widget: BorderWidget) { self.get_children_mut()[2] = Box::new(widget); }

    fn get_border_widths(&self) -> &Vector4<f32> {&self.get_border().border_widths }
    fn set_border_widths(&mut self, widths: Vector4<f32>) {
        self.get_border_mut().border_widths = widths;
        self.set_inner_transforms();
    }

    fn get_padding(&self) -> &Vector4<f32>;
    fn set_padding_inner_val(&mut self, widths: Vector4<f32>);
    fn set_padding(&mut self, widths: Vector4<f32>) {
        self.set_padding_inner_val(widths);
        self.set_inner_transforms();
    }

    fn set_inner_transforms(&mut self) {
        let padding = self.get_padding();
        let border_widths = self.get_border_widths();

        let width = 1.0 - border_widths.x - border_widths.y - padding.x - padding.y;
        let height = 1.0 - border_widths.z - border_widths.w - padding.z - padding.w;

        let x = border_widths.x + padding.x;
        let y = border_widths.z + padding.z;

        let widget = self.get_inner_widget_mut();

        widget.set_size(width, height);
        widget.set_position(vec2(x, y));
    }

    // TODO: Implement some kind of way to keep all frame sides on equal terms
    // TODO: Implement way to set frame sides in pixels

    // TODO: Add padding
}