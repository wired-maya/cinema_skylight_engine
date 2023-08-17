use silver_gl::RenderPipeline;
use crate::EngineError;

// TODO: rewrite to include children tree as well
pub trait Scene {
    fn get_size(&self) -> (i32, i32);
    fn set_size(&mut self, width: i32, height: i32) -> Result<(), EngineError>;

    fn get_render_pipeline(&self) -> &Box<dyn RenderPipeline>;
    fn get_render_pipeline_mut(&mut self) -> &mut Box<dyn RenderPipeline>;
    fn set_render_pipeline(&mut self, render_pipeline: Box<dyn RenderPipeline>);

    fn draw(&mut self) -> Result<(), EngineError>;
}