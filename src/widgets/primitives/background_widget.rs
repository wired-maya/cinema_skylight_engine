use std::rc::Rc;
use cgmath::{Vector4, Quaternion, Matrix4, SquareMatrix, Vector2};
use silver_gl::{ShaderProgram, MultiBindModel, ModelCreateTrait};
use crate::{Widget, EngineError, create_wquad};
use super::PrimitiveType;

pub struct BackgroundWidget {
    // TODO: Add colour struct that stores everything in RGB,
    // TODO: but can be made from hex, etc
    pub colour: Vector4<f32>,
    pub position: Vector2<f32>,
    pub rotation: Quaternion<f32>,
    pub width: f32,
    pub height: f32,
    pub children: Vec<Box<dyn Widget>>,
    pub vec_space: Matrix4<f32>,
    shader_program: Rc<ShaderProgram>,
    inner: MultiBindModel
}

impl BackgroundWidget {
    pub fn new(shader_program: Rc<ShaderProgram>) -> Result<Self, EngineError> {
        let model = create_wquad();

        Ok(
            Self {
                colour: Vector4::<f32>::new(0.0, 0.0, 0.0, 0.0),
                position: Vector2::<f32>::new(0.0, 0.0),
                rotation: Quaternion::<f32>::new(1.0, 0.0, 0.0, 0.0),
                width: 1.0,
                height: 1.0,
                children: Default::default(),
                vec_space: Matrix4::<f32>::identity(),
                shader_program
            }
        )
    }
}

impl Widget for BackgroundWidget {
    fn get_position(&self) -> Vector2<f32> { self.position }
    fn set_position(&mut self, pos: Vector2<f32>) { self.position = pos }
    fn get_rotation(&self) -> Quaternion<f32> { self.rotation }
    fn set_rotation(&mut self, rot: Quaternion<f32>) { self.rotation = rot }
    fn get_size(&self) -> (f32, f32) { (self.width, self.height) }
    fn set_size(&mut self, width: f32, height: f32) { self.width = width; self.height = height }
    fn get_children(&self) -> &Vec<Box<dyn Widget>> { &self.children }
    fn get_children_mut(&mut self) -> &mut Vec<Box<dyn Widget>> { &mut self.children }
    fn get_index(&self) -> Option<usize> { self.index }
    fn set_index(&mut self, i: Option<usize>) { self.index = i }
    fn get_vec_space(&self) -> Matrix4<f32> { self.vec_space }
    fn set_vec_space(&mut self, vec_space: Matrix4<f32>) { self.vec_space = vec_space }

    // fn widget_info(&mut self) -> Vec<u8> {
    //     let mut data: Vec<u8> = Vec::new();
            
    //     data.extend((PrimitiveType::Background as u32).to_ne_bytes());
    //     data.extend(self.colour.x.to_ne_bytes());
    //     data.extend(self.colour.y.to_ne_bytes());
    //     data.extend(self.colour.z.to_ne_bytes());
    //     data.extend(self.colour.w.to_ne_bytes());

    //     data
    // }
}