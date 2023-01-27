use std::rc::Rc;

use silver_gl::{RenderPipeline, Model, ShaderProgram, Scene};
use crate::{Camera, EngineError, ResourceManager, ShaderPathBundle, CameraSize, create_wquad, BackgroundWidget};

pub struct Widget2dScene {
    pub widget_quad: Model,
    pub widget_shader_program: Rc<ShaderProgram>,
    pub render_pipeline: Box<dyn RenderPipeline>,
    pub top_widget: BackgroundWidget,
    pub camera: Camera,
    
}

impl Widget2dScene {
    pub fn new(
        resource_manager: &mut ResourceManager,
        widget_shader_paths: ShaderPathBundle,
        camera_bundle: CameraSize,
        render_pipeline: Box<dyn RenderPipeline>
    ) -> Result<Widget2dScene, EngineError> {
        let widget_shader_program = resource_manager.load_shader_program(widget_shader_paths)?;
        let camera = Camera::new(
            camera_bundle.width,
            camera_bundle.height,
            camera_bundle.fov,
            vec![&widget_shader_program]
        )?;

        Ok(
            Widget2dScene {
                widget_quad: create_wquad(),
                widget_shader_program,
                render_pipeline,
                top_widget: BackgroundWidget::new(),
                camera
            }
        )
    }
}

impl Scene for Widget2dScene {
    fn set_size(&mut self, width: i32, height: i32) -> Result<(), silver_gl::GlError> {
        self.render_pipeline.set_size(width, height)?;
        self.camera.width = width as f32;
        self.camera.height = height as f32;
        self.camera.send_proj()?;

        Ok(())
    }

    fn draw(&mut self) -> Result<(), silver_gl::GlError> {
        unsafe { gl::Enable(gl::DEPTH_TEST) };

        self.camera.send_view()?;

        self.render_pipeline.bind();
        self.widget_shader_program.use_program();

        self.widget_quad.draw(&self.widget_shader_program)?;

        self.render_pipeline.draw()?;

        Ok(())
    }
}