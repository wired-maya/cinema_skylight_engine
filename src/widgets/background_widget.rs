use cgmath::{Vector4, Quaternion, Matrix4, SquareMatrix, Vector3};
use crate::{Widget, EngineError};
use silver_gl::ShaderProgram;

pub struct BackgroundWidget {
    pub colour: Vector4<f32>,
    pub position: Vector3<f32>,
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
            position: Vector3::<f32>::new(0.0, 0.0, 0.0),
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
    fn get_position(&self) -> Vector3<f32> { self.position }
    fn get_rotation(&self) -> cgmath::Quaternion<f32> { self.rotation }
    fn get_size(&self) -> (f32, f32) { (self.width, self.height) }
    fn get_children(&self) -> &Vec<Box<dyn Widget>> { &self.children }
    fn get_children_mut(&mut self) -> &mut Vec<Box<dyn Widget>> { &mut self.children }
    fn get_index(&self) -> Option<usize> { self.index }
    fn set_index(&mut self, i: Option<usize>) { self.index = i }
    fn get_vec_space(&self) -> cgmath::Matrix4<f32> { self.vec_space }
    fn set_vec_space(&mut self, vec_space: cgmath::Matrix4<f32>) { self.vec_space = vec_space }

    fn send_widget_info(&self, shader_program: &ShaderProgram) -> Result<(), crate::EngineError> {
        if let Some(index) = self.index {
            let name = format!("BackgroundWidgets[{}]", index);
            shader_program.set_vector_4(name.as_str(), &self.colour)?;
        } else {
            return Err(EngineError::WidgetIndexMissing());
        }

        Ok(())
    }
}