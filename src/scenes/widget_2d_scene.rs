use cgmath::{Matrix4, SquareMatrix};
use silver_gl::{RenderPipeline, gl};
use crate::{EngineError, Widget, Scene, Widget2dRenderPipeline};

pub struct Widget2dScene {
    pub children: Vec<Box<dyn Widget>>,
    pub render_pipeline: Box<dyn RenderPipeline>,
    width: i32,
    height: i32
}

impl Widget2dScene {
    pub fn new(width: i32, height: i32) -> Result<Self, EngineError> {
        Ok(
            Self {
                render_pipeline: Box::new(Widget2dRenderPipeline::new(width, height)?),
                children: Vec::new(),
                width,
                height
            }
        )
    }
}

impl Scene for Widget2dScene {
    fn get_size(&self) -> (i32, i32) { (self.width, self.height) }
    fn set_size(&mut self, width: i32, height: i32) -> Result<(), EngineError> {
        self.render_pipeline.set_size(width, height)?;
        
        Ok(())
    }

    fn get_render_pipeline(&self) -> &Box<dyn RenderPipeline> { &self.render_pipeline }
    fn get_render_pipeline_mut(&mut self) -> &mut Box<dyn RenderPipeline> { &mut self.render_pipeline }
    fn set_render_pipeline(&mut self, render_pipeline: Box<(dyn RenderPipeline + 'static)>) { self.render_pipeline = render_pipeline }

    fn draw(&mut self) -> Result<(), EngineError> {
        unsafe { gl::Disable(gl::DEPTH_TEST) };

        self.render_pipeline.bind();

        for widget in &mut self.children {
            widget.draw(Matrix4::identity())?;
        }

        self.render_pipeline.draw()?;

        Ok(())
    }
}