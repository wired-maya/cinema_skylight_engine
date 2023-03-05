use cgmath::{Vector2, Quaternion, Matrix4, SquareMatrix, Vector4};
use silver_gl::{ShaderProgram, Model};
use crate::{Widget, primitives::{BorderWidget, BackgroundWidget, TextureWidget, PrimitiveCounter}, EngineError, FramedWidget, ResourceManager};

pub struct PictureWidget {
    pub position: Vector2<f32>,
    pub rotation: Quaternion<f32>,
    pub width: f32,
    pub height: f32,
    pub vec_space: Matrix4<f32>,
    pub padding: Vector4<f32>,
    children: Vec<Box<dyn Widget>>
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
                Box::new(TextureWidget {
                    position: Vector2::<f32>::new(0.06, 0.06),
                    width: 0.88,
                    height: 0.88,
                    ..Default::default()
                }),
                Box::new(BorderWidget {
                    border_widths: Vector4::<f32>::new(0.01, 0.01, 0.01, 0.01),
                    ..Default::default()
                })
            ],
            vec_space: Matrix4::<f32>::identity(),
            padding: Vector4::<f32>::new(0.05, 0.05, 0.05, 0.05)
        }
    }
}

impl PictureWidget {
    pub fn from_path(resource_manager: &mut ResourceManager, path: &str) -> Result<PictureWidget, EngineError> {
        let texture = resource_manager.load_texture_2d(path)?;

        Ok(
            PictureWidget {
                children: vec![
                    Box::new(BackgroundWidget::default()),
                    Box::new(TextureWidget {
                        texture: Some(texture),
                        position: Vector2::<f32>::new(0.06, 0.06),
                        width: 0.88,
                        height: 0.88,
                        ..Default::default()
                    }),
                    Box::new(BorderWidget {
                        border_widths: Vector4::<f32>::new(0.01, 0.01, 0.01, 0.01),
                        ..Default::default()
                    })
                ],
                ..Default::default()
            }
        )
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
    fn get_index(&self) -> Option<usize> { None }
    fn set_index(&mut self, _: Option<usize>) { }
    fn get_vec_space(&self) -> Matrix4<f32> { self.vec_space }
    fn set_vec_space(&mut self, vec_space: Matrix4<f32>) { self.vec_space = vec_space }

    fn send_widget_info(&self, _: &ShaderProgram, _: &mut PrimitiveCounter) -> Result<(), EngineError> { Ok(()) }

    // Composite widgets only hold primitives and their vector space, so it doesn't
    // send anything to the quad
    fn traverse_and_push_all(
        &mut self,
        quad: &mut Model,
        shader_program: &ShaderProgram,
        vec_space: Matrix4<f32>,
        counter: &mut PrimitiveCounter
    ) -> Result<(), EngineError> {
        self.set_vec_space(vec_space);
        let matrix = self.transform_matrix();

        for widget in self.get_children_mut() {
            widget.traverse_and_push_all(quad, shader_program, matrix, counter)?;
        }

        Ok(())
    }

    fn traverse_and_set_transforms(&mut self, quad: &mut Model, vec_space: Matrix4<f32>) -> Result<(), EngineError> {
        self.set_vec_space(vec_space);
        let matrix = self.transform_matrix();

        for widget in self.get_children_mut() {
            widget.traverse_and_set_transforms(quad, matrix)?;
        }
        
        Ok(())
    }

    fn traverse_and_set_textures(&self, quad: &mut Model) -> Result<(), EngineError> {
        for widget in self.get_children() {
            widget.traverse_and_set_textures(quad)?;
        }
        
        Ok(())
    }

    fn set_transform(&self, _: &mut Model) -> Result<(), EngineError> { Err(EngineError::WidgetNotPrimitive()) }
    fn set_transform_send(&self, _: &mut Model) -> Result<(), EngineError> { Err(EngineError::WidgetNotPrimitive()) }
    fn set_texture_send(&self, _: &mut Model) -> Result<(), EngineError> { Err(EngineError::WidgetNotPrimitive()) }
}