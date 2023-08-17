use std::rc::Rc;
use cgmath::{Quaternion, Vector2, Matrix4, vec2, SquareMatrix};
use silver_gl::{ShaderProgram, MultiBindModel};
use crate::{Widget, EngineError, create_wquad};

pub struct ContainerWidget {
    pub position: Vector2<f32>,
    pub rotation: Quaternion<f32>,
    pub width: f32,
    pub height: f32,
    pub children: Vec<Box<dyn Widget>>,
    pub shader_program: Rc<ShaderProgram>,
    pub model: MultiBindModel,
    pub vec_space: Matrix4<f32>,
}

impl ContainerWidget {
    pub fn new(shader_program: Rc<ShaderProgram>) -> Self {
        Self {
            position: vec2(0.0, 0.0),
            rotation: Quaternion::new(1.0, 0.0, 0.0, 0.0),
            width: 1.0,
            height: 1.0,
            children: Vec::new(),
            shader_program,
            model: create_wquad(),
            vec_space: Matrix4::identity()
        }
    }
}

impl Widget for ContainerWidget {
    fn get_position(&self) -> Vector2<f32> { self.position }
    fn set_position(&mut self, pos: Vector2<f32>) { self.position = pos }

    fn get_rotation(&self) -> Quaternion<f32> { self.rotation }
    fn set_rotation(&mut self, rot: Quaternion<f32>) { self.rotation = rot }
    
    fn get_size(&self) -> (f32, f32) { (self.width, self.height) }
    fn set_size(&mut self, width: f32, height: f32) { self.width = width; self.height = height }

    fn get_children(&self) -> &Vec<Box<dyn Widget>> { &self.children }
    fn get_children_mut(&mut self) -> &mut Vec<Box<dyn Widget>> { &mut self.children }

    fn get_shader_program(&self) -> &Rc<ShaderProgram> { &self.shader_program }
    fn set_shader_program(&mut self, shader_program: Rc<ShaderProgram>) { self.shader_program = shader_program }

    fn get_model(&self) -> &MultiBindModel { &self.model }
    fn get_model_mut(&mut self) -> &mut MultiBindModel { &mut self.model }
    fn set_model(&mut self, model: MultiBindModel) { self.model = model }

    fn get_vec_space(&self) -> Matrix4<f32> { self.vec_space }
    fn set_vec_space(&mut self, vec_space: Matrix4<f32>) { self.vec_space = vec_space }
    
    fn update_shader_program(&self) -> Result<(), EngineError> { Ok(()) }
}