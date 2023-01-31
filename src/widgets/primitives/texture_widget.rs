use std::rc::Rc;

use cgmath::{Vector2, Quaternion, Matrix4, SquareMatrix};
use silver_gl::Texture;

use crate::{Widget, EngineError};

use super::PrimitiveType;

pub struct TextureWidget {
    pub position: Vector2<f32>,
    pub rotation: Quaternion<f32>,
    pub width: f32,
    pub height: f32,
    pub children: Vec<Box<dyn Widget>>,
    pub index: Option<usize>,
    pub vec_space: Matrix4<f32>,
    pub texture: Option<Rc<Texture>>
}

impl Default for TextureWidget {
    fn default() -> Self {
        Self {
            position: Vector2::<f32>::new(0.0, 0.0),
            rotation: Quaternion::<f32>::new(1.0, 0.0, 0.0, 0.0),
            width: Default::default(),
            height: Default::default(),
            children: Default::default(),
            index: None,
            vec_space: Matrix4::<f32>::identity(),
            texture: None
        }
    }
}

impl Widget for TextureWidget {
    fn get_position(&self) -> Vector2<f32> { self.position }
    fn set_position(&mut self, pos: Vector2<f32>) { self.position = pos }
    fn get_rotation(&self) -> cgmath::Quaternion<f32> { self.rotation }
    fn set_rotation(&mut self, rot: Quaternion<f32>) { self.rotation = rot }
    fn get_size(&self) -> (f32, f32) { (self.width, self.height) }
    fn set_size(&mut self, width: f32, height: f32) { self.width = width; self.height = height }
    fn get_children(&self) -> &Vec<Box<dyn Widget>> { &self.children }
    fn get_children_mut(&mut self) -> &mut Vec<Box<dyn Widget>> { &mut self.children }
    fn get_index(&self) -> Option<usize> { self.index }
    fn set_index(&mut self, i: Option<usize>) { self.index = i }
    fn get_vec_space(&self) -> cgmath::Matrix4<f32> { self.vec_space }
    fn set_vec_space(&mut self, vec_space: cgmath::Matrix4<f32>) { self.vec_space = vec_space }
    fn get_texture(&self) -> &Option<std::rc::Rc<silver_gl::Texture>> { &self.texture }
    fn set_texture(&mut self, texture: Rc<Texture>) -> Result<(), crate::EngineError> { Ok(self.texture = Some(texture)) }

    fn send_widget_info(&self, shader_program: &silver_gl::ShaderProgram, counter: &mut super::PrimitiveCounter) -> Result<(), crate::EngineError> {
        if let Some(index) = self.index {
            let widget_type_str = format!("widgets[{}].type", index);
            let widget_index_str = format!("widgets[{}].index", index);
                        
            shader_program.set_int(&widget_type_str, PrimitiveType::Texture as i32)?;
            shader_program.set_int(&widget_index_str, counter.texture_num)?;

            counter.texture_num += 1;
        } else {
            return Err(EngineError::WidgetIndexMissing());
        }

        Ok(())
    }
}