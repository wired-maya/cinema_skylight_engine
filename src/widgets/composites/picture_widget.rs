use std::rc::Rc;
use cgmath::{Vector4, Quaternion, Vector2, Matrix4, vec2, SquareMatrix};
use silver_gl::{ShaderProgram, MultiBindModel, Texture};
use crate::{Widget, EngineError, primitives::{TextureWidget, BackgroundWidget, BorderWidget}, FramedWidget, create_wquad};

pub struct PictureWidget {
    pub position: Vector2<f32>,
    pub rotation: Quaternion<f32>,
    pub width: f32,
    pub height: f32,
    pub children: Vec<Box<dyn Widget>>,
    pub shader_program: Rc<ShaderProgram>,
    pub model: MultiBindModel,
    pub vec_space: Matrix4<f32>,

    pub padding: Vector4<f32>,
}

impl PictureWidget {
    pub fn new(
        picture_shader_program: Rc<ShaderProgram>,
        background_shader_program: Rc<ShaderProgram>,
        texture_shader_program: Rc<ShaderProgram>,
        border_shader_program: Rc<ShaderProgram>,
        padding: Vector4<f32>,
        background_colour: Vector4<f32>,
        texture: Rc<Texture>,
        border_colour: Vector4<f32>,
        border_widths: Vector4<f32>
    ) -> Self {
        Self {
            position: vec2(0.0, 0.0),
            rotation: Quaternion::new(1.0, 0.0, 0.0, 0.0),
            width: 1.0,
            height: 1.0,
            children: vec![
                Box::new(BackgroundWidget::new(background_shader_program, background_colour)),
                Box::new(TextureWidget::new(texture_shader_program, texture)),
                Box::new(BorderWidget::new(border_shader_program, border_colour, border_widths))
            ],
            shader_program: picture_shader_program,
            model: create_wquad(),
            vec_space: Matrix4::identity(),
            padding
        }
    }
}

impl FramedWidget for PictureWidget {
    type Inner = TextureWidget;

    fn get_padding(&self) -> &Vector4<f32> { &self.padding }
    fn set_padding_inner_val(&mut self, widths: Vector4<f32>) { self.padding = widths }
}

impl Widget for PictureWidget {
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