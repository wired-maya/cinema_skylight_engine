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
    pub texture: Option<Rc<Texture>>,
}

impl Default for TextureWidget {
    fn default() -> Self {
        Self {
            position: Vector2::<f32>::new(0.0, 0.0),
            rotation: Quaternion::<f32>::new(1.0, 0.0, 0.0, 0.0),
            width: 1.0,
            height: 1.0,
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
    fn get_texture(&self) -> &Option<Rc<Texture>> { &self.texture }
    fn set_texture(&mut self, texture: Rc<Texture>) -> Result<(), EngineError> { Ok(self.texture = Some(texture)) }

    fn widget_info(&mut self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();

        if let Some(tex) = &self.texture {
            data.extend((PrimitiveType::Texture as u32).to_ne_bytes());

            unsafe {
                let handle = tex.get_handle();
                tex.make_resident();
                data.extend(handle.to_ne_bytes());
            }
        }

        data
    }
}