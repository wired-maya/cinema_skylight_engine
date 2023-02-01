use cgmath::{Vector4, Quaternion, Matrix4, SquareMatrix, Vector2};
use crate::{Widget, EngineError};
use silver_gl::ShaderProgram;
use super::PrimitiveType;

pub struct BackgroundWidget {
    pub colour: Vector4<f32>,
    pub position: Vector2<f32>,
    pub rotation: Quaternion<f32>,
    pub width: f32,
    pub height: f32,
    pub children: Vec<Box<dyn Widget>>,
    pub index: Option<usize>,
    pub vec_space: Matrix4<f32>
}

impl Default for BackgroundWidget {
    fn default() -> Self {
        Self {
            colour: Vector4::<f32>::new(0.0, 0.0, 0.0, 0.0),
            position: Vector2::<f32>::new(0.0, 0.0),
            rotation: Quaternion::<f32>::new(1.0, 0.0, 0.0, 0.0),
            width: Default::default(),
            height: Default::default(),
            children: Default::default(),
            index: None,
            vec_space: Matrix4::<f32>::identity()
        }
    }
}

impl Widget for BackgroundWidget {
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

    fn send_widget_info(&self, shader_program: &ShaderProgram, counter: &mut super::PrimitiveCounter) -> Result<(), EngineError> {
        if let Some(index) = self.index {
            let widget_type_str = format!("widgets[{}].type", index);
            let widget_index_str = format!("widgets[{}].index", index);
            let primitive_str = format!("backgroundWidgets[{}]", counter.background_num);
            
            shader_program.set_int(&widget_type_str, PrimitiveType::Background as i32)?;
            shader_program.set_int(&widget_index_str, counter.background_num)?;
            shader_program.set_vector_4(&primitive_str, &self.colour)?;

            counter.background_num += 1;
        } else {
            return Err(EngineError::WidgetIndexMissing());
        }

        Ok(())
    }
}