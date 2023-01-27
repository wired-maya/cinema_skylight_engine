use std::rc::Rc;

use cgmath::{Vector3, Quaternion, Matrix4};
use silver_gl::{ShaderProgram, GlError, Model, Texture};

use crate::EngineError;

// TODO: Widgets are similar to gameobjects, where they can store a sub-widget that is relative to
// TODO: its parent. This is mainly going to be used to compose widgets from primitive widgets,
// TODO: for example a TextBoxWidget will be a BackgroundWidget holding a text widget and
// TODO: a border widget. Each widget will have an array of sub-widgets allowed (just like
// TODO: the gameobjects), and will be rendered in the following order: Back-most first,
// TODO: then the first child, its children, and the next child, and so on. All widgets will
// TODO: be stored in a top-level widget, same with the game objects.
// TODO: For example with this rendering, say you have a top-level background widget, with two
// TODO: textboxes. The tree is as follows:
// TODO: Background (top-level) -> TextBox1 (empty container widget) -> Background1 -> Text1
// TODO:                                                                            -> Border1
// TODO:                        -> TextBox2 -> Background2 -> Text2
// TODO:                                                   -> Border2
// TODO: This will be rendered in the following order, from bottom to top visually (primitive
// TODO: widgets are only rendered as the rest are just empty containers for organization):
// TODO: Background (top-level) -> Background1 -> Text1 -> Border1 -> Background2 -> Text2 -> Border2
// TODO: These will be held in a View2dScene which holds a WidgetQuad (the shape all widgets are
// TODO: based on, technically could be any shape if you want to get creative since it will hold
// TODO: a model), which has a bunch of transforms and draws the primitive widgets with instanced
// TODO: rendering. To achieve the proper render order without Z-fighting, each time the widget tree
// TODO: is modified, a function traverses the widget tree, orders them all in a Vec, and
// TODO: transforms each widget an equal distance between [0.0, -1.0] on z. Drawing will then
// TODO: just be as simple as drawing the one quad, with all textures bound in the correct order.
pub trait Widget {
    fn get_position(&self) -> Vector3<f32>;
    fn get_rotation(&self) -> Quaternion<f32>;
    fn get_size(&self) -> (f32, f32);
    fn get_texture(&self) -> Option<Rc<Texture>> { None } // Used for texture primitive widgets
    fn get_children(&self) -> &Vec<Box<dyn Widget>>;
    fn transform_matrix(&self) -> Matrix4<f32> {
        let mut matrix = Matrix4::<f32>::from_translation(self.get_position());
        let (width, height) = self.get_size();
        matrix = matrix * Matrix4::<f32>::from_nonuniform_scale(width, height, 1.0);
        matrix = matrix * Matrix4::<f32>::from(self.get_rotation());
        
        matrix
    }
    fn send_widget_info(&self) -> Result<(), EngineError>;
    // Traverses tree in linearized order, pushing the widgets' information to the quad
    // Assumes that clear_inner has been run on the tbo and clear has been run on the
    // mesh's diffuse textures, that send_data_mut is ran afterwards, and that a shader
    // program is bound.
    // Panics if there are less than 1 meshes in quad.
    fn traverse_and_push(&self, quad: &mut Model) -> Result<(), EngineError> {
        unsafe {
            quad.tbo.push_to_inner(self.transform_matrix());
        }

        if let Some(texture) = self.get_texture() {
            quad.meshes[0].diffuse_textures.push(texture);
        }

        self.send_widget_info()?;

        for widget in self.get_children() {
            widget.traverse_and_push(quad)?;
        }

        Ok(())
    }
}