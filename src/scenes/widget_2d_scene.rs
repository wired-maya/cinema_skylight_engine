use std::{rc::Rc};
use cgmath::{Matrix4, SquareMatrix};
use silver_gl::{RenderPipeline, ShaderProgram, gl};
use crate::{Camera, EngineError, ResourceManager, ShaderPathBundle, CameraSize, create_wquad, Widget, Scene, CameraProjection, widget_model::WModel};

pub struct Widget2dScene {
    pub shader_program: Rc<ShaderProgram>,
    pub render_pipeline: Box<dyn RenderPipeline>,
    pub widget: Box<dyn Widget>
}

impl Widget2dScene {
    pub fn new(
        resource_manager: &mut ResourceManager,
        shader_paths: ShaderPathBundle,
        render_pipeline: Box<dyn RenderPipeline>,
        widget: Box<dyn Widget>
    ) -> Result<Widget2dScene, EngineError> {
        let shader_program = resource_manager.load_shader_program(shader_paths)?;

        Ok(
            Widget2dScene {
                shader_program,
                render_pipeline,
                widget
            }
        )
    }
}

impl Scene for Widget2dScene {
    fn set_size(&mut self, width: i32, height: i32) -> Result<(), EngineError> {
        self.render_pipeline.set_size(width, height)?;
        self.widget.set_size(width as f32, height as f32);

        // TODO: adjust size of all widgets

        Ok(())
    }

    fn draw(&mut self) -> Result<(), EngineError> {
        unsafe { gl::Disable(gl::DEPTH_TEST) };

        self.render_pipeline.bind();
        self.shader_program.use_program();

        self.widget.draw();
        self.render_pipeline.draw()?;

        Ok(())
    }
}