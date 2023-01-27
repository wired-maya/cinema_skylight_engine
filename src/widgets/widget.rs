use std::rc::Rc;

use cgmath::{Vector3, Quaternion, Matrix4, vec3};
use silver_gl::{Model, Texture, ShaderProgram};

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
    fn get_position(&self) -> Vector3<i32>;
    fn get_rotation(&self) -> Quaternion<f32>;
    fn get_size(&self) -> (i32, i32);
    fn get_texture(&self) -> Option<Rc<Texture>> { None } // Used for texture primitive widgets
    fn get_children(&self) -> &Vec<Box<dyn Widget>>;
    fn get_children_mut(&mut self) -> &mut Vec<Box<dyn Widget>>;
    fn transform_matrix(&self) -> Matrix4<f32> {
        let pos = self.get_position();
        let mut matrix = Matrix4::<f32>::from_translation(vec3(pos.x as f32, pos.y as f32, pos.z as f32));
        let (width, height) = self.get_size();
        matrix = matrix * Matrix4::<f32>::from_nonuniform_scale(width as f32, height as f32, 1.0);
        matrix = matrix * Matrix4::<f32>::from(self.get_rotation());
        
        matrix
    }

    // These are used to optimize changing textures and transforms
    fn get_index(&self) -> Option<usize>;
    fn set_index(&mut self, i: Option<usize>);

    // Send visual properties of the widget to shader program
    fn send_widget_info(&self, shader_program: &ShaderProgram) -> Result<(), EngineError>;

    // Traverses tree in linearized order, pushing the widgets' information to the quad
    // Assumes that clear_inner has been run on the tbo and clear has been run on the
    // mesh's diffuse textures, that send_data_mut is ran afterwards, and that a shader
    // program is bound.
    // Panics if there are less than 1 meshes in quad.
    // Needs to be run when widget tree is changed
    fn traverse_and_push_all(&mut self, quad: &mut Model, shader_program: &ShaderProgram) -> Result<(), EngineError> {
        self.set_index(Some(quad.tbo.len()));
        
        unsafe {
            quad.tbo.push_to_inner(self.transform_matrix());
        }

        if let Some(texture) = self.get_texture() {
            quad.meshes[0].diffuse_textures.push(texture);
        }

        self.send_widget_info(shader_program)?;

        for widget in self.get_children_mut() {
            widget.traverse_and_push_all(quad, shader_program)?;
        }

        Ok(())
    }

    // Index needs to be set, meaning above function needs to have been run
    // Cannot be run after widget tree is modified without push_all being run first
    // To Used to batch together transforms
    // This is meant to to be run with a send_data afterwards, since it's to batch
    // transforms
    fn traverse_and_set_transforms(&self, quad: &mut Model) -> Result<(), EngineError> {
        if let Some(index) = self.get_index() {
            unsafe {
                quad.tbo.set_data_index_inner(self.transform_matrix(), index);
            }
        } else {
            return Err(EngineError::WidgetIndexMissing());
        }

        for widget in self.get_children() {
            widget.traverse_and_set_transforms(quad)?;
        }
        
        Ok(())
    }

    fn traverse_and_set_textures(&self, quad: &mut Model) -> Result<(), EngineError> {
        if let (Some(index), Some(texture)) = (self.get_index(), self.get_texture()) {
            quad.meshes[0].diffuse_textures[index] = texture;
        } else {
            return Err(EngineError::WidgetIndexMissing());
        }

        for widget in self.get_children() {
            widget.traverse_and_set_textures(quad)?;
        }
        
        Ok(())
    }

    // These set inner transforms and textures, so you can batch-transform a selection
    // of widgets without sacrificing performance
    // Requires a send_data to be send afterwards
    fn set_transform(&self, quad: &mut Model) -> Result<(), EngineError> {
        if let Some(index) = self.get_index() {
            unsafe {
                quad.tbo.set_data_index_inner(self.transform_matrix(), index);
            }
        } else {
            return Err(EngineError::WidgetIndexMissing());
        }
        
        Ok(())
    }

    // Sets index properties
    // This are meant to be run standalone, so only when updating one thing
    // If performance is an issue, use one of the above
    // Doesn't have all in one transform because of the 3 things that make it up
    fn set_transform_send(&self, quad: &mut Model) -> Result<(), EngineError> {
        if let Some(index) = self.get_index() {
            quad.tbo.set_data_index(self.transform_matrix(), index);
        } else {
            return Err(EngineError::WidgetIndexMissing());
        }
        
        Ok(())
    }

    // Textures are always sent to the shader each frame, so this doesn't need any subsequent
    // function
    fn set_texture_send(&self, quad: &mut Model) -> Result<(), EngineError> {
        if let (Some(index), Some(texture)) = (self.get_index(), self.get_texture()) {
            quad.meshes[0].diffuse_textures[index] = texture;
        } else {
            return Err(EngineError::WidgetIndexMissing());
        }
        
        Ok(())
    }

    // All in one setter to simplify process of setting textures
    // This should be the only way to set textures, since batching isn't something you need
    // to worry about.
    // Only used for texure primitive widgets
    fn set_texture(&mut self, texture: Texture) -> Result<(), EngineError> { Err(EngineError::TexturelessWidget(texture.get_id())) }

    // Needs to be run each frame
    fn traverse_and_send_info(&self, shader_program: &ShaderProgram) -> Result<(), EngineError> {
        self.send_widget_info(shader_program)?;

        for widget in self.get_children() {
            widget.traverse_and_send_info(shader_program)?;
        }

        Ok(())
    }
}