use cgmath::{Matrix4, SquareMatrix};
use silver_gl::{RenderPipeline, gl};
use crate::{EngineError, Widget, Scene};

pub struct Widget2dScene {
    pub render_pipeline: Box<dyn RenderPipeline>,
    pub children: Vec<Box<dyn Widget>>,
    width: i32,
    height: i32
}

impl Scene for Widget2dScene {
    fn get_size(&self) -> (i32, i32) { (self.width, self.height) }
    fn set_size(&mut self, width: i32, height: i32) -> Result<(), EngineError> {
        self.render_pipeline.set_size(width, height)?;
        
        Ok(())
    }

    fn draw(&mut self) -> Result<(), EngineError> {
        unsafe { gl::Disable(gl::DEPTH_TEST) };

        self.render_pipeline.bind();

        for widget in &self.children {
            widget.draw(Matrix4::identity())?;
        }

        self.render_pipeline.draw()?;

        Ok(())
    }
}