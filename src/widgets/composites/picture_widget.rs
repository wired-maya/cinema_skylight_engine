use cgmath::{Vector2, Quaternion, Matrix4, SquareMatrix};

use crate::{Widget, primitives::{BorderWidget, BackgroundWidget, TextureWidget}};

pub struct PictureWidget {
    pub position: Vector2<f32>,
    pub rotation: Quaternion<f32>,
    pub width: f32,
    pub height: f32,
    pub children: Vec<Box<dyn Widget>>,
    pub vec_space: Matrix4<f32>
}

impl Default for PictureWidget {
    fn default() -> Self {
        Self {
            position: Vector2::<f32>::new(0.0, 0.0),
            rotation: Quaternion::<f32>::new(1.0, 0.0, 0.0, 0.0),
            width: Default::default(),
            height: Default::default(),
            children: vec![
                Box::new(BackgroundWidget::default()),
                Box::new(TextureWidget::default()),
                Box::new(BorderWidget::default())
            ],
            vec_space: Matrix4::<f32>::identity(),
        }
    }
}

// TODO: add from function that takes the 3 primitive widgets
// TODO: add new function that takes pixture, loads it as texture, and returns this

// TODO: create and implement framed widget trait

impl Widget for PictureWidget {
    fn get_position(&self) -> Vector2<f32> { self.position }
    fn set_position(&mut self, pos: Vector2<f32>) { self.position = pos }
    fn get_rotation(&self) -> cgmath::Quaternion<f32> { self.rotation }
    fn set_rotation(&mut self, rot: Quaternion<f32>) { self.rotation = rot }
    fn get_size(&self) -> (f32, f32) { (self.width, self.height) }
    fn set_size(&mut self, width: f32, height: f32) { self.width = width; self.height = height }
    fn get_children(&self) -> &Vec<Box<dyn Widget>> { &self.children }
    fn get_children_mut(&mut self) -> &mut Vec<Box<dyn Widget>> { &mut self.children }
    fn get_index(&self) -> Option<usize> { None }
    fn set_index(&mut self, i: Option<usize>) { }
    fn get_vec_space(&self) -> cgmath::Matrix4<f32> { self.vec_space }
    fn set_vec_space(&mut self, vec_space: cgmath::Matrix4<f32>) { self.vec_space = vec_space }

    fn send_widget_info(&self, shader_program: &silver_gl::ShaderProgram, counter: &mut crate::primitives::PrimitiveCounter) -> Result<(), crate::EngineError> {
        todo!()
    }

    fn get_size_pixels(&self) -> (f32, f32) {
        let (width, height) = self.get_size();
        let mut size_vec = cgmath::vec4(width, height, 0.0, 1.0);

        size_vec = self.get_vec_space() * size_vec;

        (size_vec.x, size_vec.y)
    }

    fn set_size_pixels(&mut self, width: f32, height: f32) {
        let mut size_vec = cgmath::vec4(width, height, 0.0, 1.0);

        let inverted_vec_space = self.get_vec_space().invert().expect("Transformation matrix should be invertible");
        size_vec = inverted_vec_space * size_vec;

        self.set_size(size_vec.x, size_vec.y)
    }

    fn traverse_and_push_all(
        &mut self,
        quad: &mut silver_gl::Model,
        shader_program: &silver_gl::ShaderProgram,
        vec_space: Matrix4<f32>,
        counter: &mut crate::primitives::PrimitiveCounter
    ) -> Result<(), crate::EngineError> {
        self.set_index(Some(quad.tbo.len()));
        self.set_vec_space(vec_space);
        
        let matrix = self.transform_matrix();

        unsafe {
            quad.tbo.push_to_inner(matrix);
        }

        if let Some(texture) = self.get_texture() {
            quad.meshes[0].diffuse_textures.push(std::rc::Rc::clone(texture));
        }

        self.send_widget_info(shader_program, counter)?;

        for widget in self.get_children_mut() {
            widget.traverse_and_push_all(quad, shader_program, matrix, counter)?;
        }

        Ok(())
    }

    fn traverse_and_set_transforms(&mut self, quad: &mut silver_gl::Model, vec_space: Matrix4<f32>) -> Result<(), crate::EngineError> {
        self.set_vec_space(vec_space);

        let matrix = self.transform_matrix();

        if let Some(index) = self.get_index() {
            unsafe {
                quad.tbo.set_data_index_inner(matrix, index);
            }
        } else {
            return Err(crate::EngineError::WidgetIndexMissing());
        }

        for widget in self.get_children_mut() {
            widget.traverse_and_set_transforms(quad, matrix)?;
        }
        
        Ok(())
    }

    fn traverse_and_set_textures(&self, quad: &mut silver_gl::Model) -> Result<(), crate::EngineError> {
        if let (Some(index), Some(texture)) = (self.get_index(), self.get_texture()) {
            quad.meshes[0].diffuse_textures[index] = std::rc::Rc::clone(texture);
        } else {
            return Err(crate::EngineError::WidgetIndexMissing());
        }

        for widget in self.get_children() {
            widget.traverse_and_set_textures(quad)?;
        }
        
        Ok(())
    }

    fn set_transform(&self, quad: &mut silver_gl::Model) -> Result<(), crate::EngineError> {
        if let Some(index) = self.get_index() {
            unsafe {
                quad.tbo.set_data_index_inner(self.transform_matrix(), index);
            }
        } else {
            return Err(crate::EngineError::WidgetIndexMissing());
        }
        
        Ok(())
    }

    fn set_transform_send(&self, quad: &mut silver_gl::Model) -> Result<(), crate::EngineError> {
        if let Some(index) = self.get_index() {
            quad.tbo.set_data_index(self.transform_matrix(), index);
        } else {
            return Err(crate::EngineError::WidgetIndexMissing());
        }
        
        Ok(())
    }

    fn set_texture_send(&self, quad: &mut silver_gl::Model) -> Result<(), crate::EngineError> {
        if let (Some(index), Some(texture)) = (self.get_index(), self.get_texture()) {
            quad.meshes[0].diffuse_textures[index] = std::rc::Rc::clone(texture);
        } else {
            return Err(crate::EngineError::WidgetIndexMissing());
        }
        
        Ok(())
    }

    fn get_texture(&self) -> &Option<std::rc::Rc<silver_gl::Texture>> { &None }

    fn set_texture(&mut self, texture: std::rc::Rc<silver_gl::Texture>) -> Result<(), crate::EngineError> { Err(crate::EngineError::TexturelessWidget(texture.get_id())) }

    fn traverse_and_send_info(&self, shader_program: &silver_gl::ShaderProgram, counter: &mut crate::primitives::PrimitiveCounter) -> Result<(), crate::EngineError> {
        self.send_widget_info(shader_program, counter)?;

        for widget in self.get_children() {
            widget.traverse_and_send_info(shader_program, counter)?;
        }

        Ok(())
    }
}