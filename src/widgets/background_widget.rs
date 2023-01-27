use cgmath::{Vector4, Vector3, Quaternion, vec3};
use crate::{Widget, EngineError};
use silver_gl::ShaderProgram;

pub struct BackgroundWidget {
    pub colour: Vector4<f32>,
    pub position: Vector3<i32>,
    pub rotation: Quaternion<f32>,
    pub width: i32,
    pub height: i32,
    pub children: Vec<Box<dyn Widget>>,
    pub index: Option<usize>
}

impl Default for BackgroundWidget {
    fn default() -> Self {
        Self {
            colour: Vector4::<f32>::new(0.0, 0.0, 0.0, 0.0),
            position: Vector3::<i32>::new(0, 0, 0),
            rotation: Quaternion::<f32>::new(1.0, 0.0, 0.0, 0.0),
            width: Default::default(),
            height: Default::default(),
            children: Default::default(),
            index: None
        }
    }
}

impl Widget for BackgroundWidget {
    fn get_position(&self) -> Vector3<i32> { self.position }
    fn get_rotation(&self) -> cgmath::Quaternion<f32> { self.rotation }
    fn get_size(&self) -> (i32, i32) { (self.width, self.height) }
    fn get_children(&self) -> &Vec<Box<dyn Widget>> { &self.children }
    fn get_children_mut(&mut self) -> &mut Vec<Box<dyn Widget>> { &mut self.children }
    fn get_index(&self) -> Option<usize> { self.index }
    fn set_index(&mut self, i: Option<usize>) { self.index = i }

    fn send_widget_info(&self, shader_program: &ShaderProgram) -> Result<(), crate::EngineError> {
        if let Some(index) = self.index {
            let pos = self.position;
            let name = format!("BackgroundWidgets[{}]", index);
            shader_program.set_vector_3(name.as_str(), &vec3(pos.x as f32, pos.y as f32, pos.z as f32))?;
        } else {
            return Err(EngineError::WidgetIndexMissing());
        }

        Ok(())
    }
}