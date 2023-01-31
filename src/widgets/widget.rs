use std::rc::Rc;

use cgmath::{Quaternion, Matrix4, Vector2, vec3, vec4, SquareMatrix};
use silver_gl::{Model, Texture, ShaderProgram};

use crate::{EngineError, primitives::PrimitiveCounter};

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
    fn get_position(&self) -> Vector2<f32>;
    fn set_position(&mut self, pos: Vector2<f32>);
    fn get_rotation(&self) -> Quaternion<f32>;
    fn set_rotation(&mut self, rot: Quaternion<f32>);
    fn get_size(&self) -> (f32, f32);
    fn set_size(&mut self, width: f32, height: f32);
    fn get_children(&self) -> &Vec<Box<dyn Widget>>;
    fn get_children_mut(&mut self) -> &mut Vec<Box<dyn Widget>>;

    // Calculates transform matrix for the vertex shader
    fn transform_matrix(&self) -> Matrix4<f32> {
        let pos = self.get_position();
        let mut matrix = Matrix4::<f32>::from_translation(vec3(pos.x, pos.y, 0.0));
        let (width, height) = self.get_size();
        matrix = matrix * Matrix4::<f32>::from_nonuniform_scale(width, height, 1.0);
        // TODO: Add option to change where the widget is rotated from
        matrix = matrix * Matrix4::<f32>::from(self.get_rotation());
        matrix = self.get_vec_space() * matrix;
        
        matrix
    }
    
    // Transforms current size to pixels
    fn get_size_pixels(&self) -> (f32, f32) {
        let (width, height) = self.get_size();
        let mut size_vec = vec4(width, height, 0.0, 1.0);

        size_vec = self.get_vec_space() * size_vec;

        (size_vec.x, size_vec.y)
    }

    fn set_size_pixels(&mut self, width: f32, height: f32) {
        let mut size_vec = vec4(width, height, 0.0, 1.0);

        let inverted_vec_space = self.get_vec_space().invert().expect("Transformation matrix should be invertible");
        size_vec = inverted_vec_space * size_vec;

        self.set_size(size_vec.x, size_vec.y)
    }

    // These are used to optimize changing textures and transforms
    fn get_index(&self) -> Option<usize>;
    fn set_index(&mut self, i: Option<usize>);

    // Used for relative widgets
    fn get_vec_space(&self) -> Matrix4<f32>;
    fn set_vec_space(&mut self, vec_space: Matrix4<f32>);

    // Send visual properties of the widget to shader program
    fn send_widget_info(&self, shader_program: &ShaderProgram, counter: &mut PrimitiveCounter) -> Result<(), EngineError>;

    // Traverses tree in linearized order, pushing the widgets' information to the quad
    // Assumes that clear_inner has been run on the tbo and clear has been run on the
    // mesh's diffuse textures, that send_data_mut is ran afterwards, and that a shader
    // program is bound.
    // Panics if there are less than 1 meshes in quad.
    // Needs to be run when widget tree is changed
    fn traverse_and_push_all(
        &mut self,
        quad: &mut Model,
        shader_program: &ShaderProgram,
        vec_space: Matrix4<f32>,
        counter: &mut PrimitiveCounter
    ) -> Result<(), EngineError> {
        self.set_index(Some(quad.tbo.len()));
        self.set_vec_space(vec_space);
        
        let matrix = self.transform_matrix();

        unsafe {
            quad.tbo.push_to_inner(matrix);
        }

        if let Some(texture) = self.get_texture() {
            quad.meshes[0].diffuse_textures.push(Rc::clone(texture));
        }

        self.send_widget_info(shader_program, counter)?;

        for widget in self.get_children_mut() {
            widget.traverse_and_push_all(quad, shader_program, matrix, counter)?;
        }

        Ok(())
    }

    // Index needs to be set, meaning above function needs to have been run
    // Cannot be run after widget tree is modified without push_all being run first
    // To Used to batch together transforms
    // This is meant to to be run with a send_data afterwards, since it's to batch
    // transforms
    fn traverse_and_set_transforms(&mut self, quad: &mut Model, vec_space: Matrix4<f32>) -> Result<(), EngineError> {
        self.set_vec_space(vec_space);

        let matrix = self.transform_matrix();

        if let Some(index) = self.get_index() {
            unsafe {
                quad.tbo.set_data_index_inner(matrix, index);
            }
        } else {
            return Err(EngineError::WidgetIndexMissing());
        }

        for widget in self.get_children_mut() {
            widget.traverse_and_set_transforms(quad, matrix)?;
        }
        
        Ok(())
    }

    fn traverse_and_set_textures(&self, quad: &mut Model) -> Result<(), EngineError> {
        if let (Some(index), Some(texture)) = (self.get_index(), self.get_texture()) {
            quad.meshes[0].diffuse_textures[index] = Rc::clone(texture);
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
            quad.meshes[0].diffuse_textures[index] = Rc::clone(texture);
        } else {
            return Err(EngineError::WidgetIndexMissing());
        }
        
        Ok(())
    }

    // All in one setter to simplify process of setting textures
    // This should be the only way to set textures, since batching isn't something you need
    // to worry about.
    // Only used for texure primitive widgets
    fn get_texture(&self) -> &Option<Rc<Texture>> { &None } // Used for texture primitive widgets
    fn set_texture(&mut self, texture: Rc<Texture>) -> Result<(), EngineError> { Err(EngineError::TexturelessWidget(texture.get_id())) }

    // Needs to be run each frame
    fn traverse_and_send_info(&self, shader_program: &ShaderProgram, counter: &mut PrimitiveCounter) -> Result<(), EngineError> {
        self.send_widget_info(shader_program, counter)?;

        for widget in self.get_children() {
            widget.traverse_and_send_info(shader_program, counter)?;
        }

        Ok(())
    }
}