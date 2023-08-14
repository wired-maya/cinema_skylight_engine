use std::{rc::Rc};
use cgmath::{Matrix4, SquareMatrix};
use silver_gl::{RenderPipeline, ShaderProgram, gl};
use crate::{Camera, EngineError, ResourceManager, ShaderPathBundle, CameraSize, create_wquad, Widget, Scene, CameraProjection, widget_model::WModel};

pub struct Widget2dScene {
    pub shader_program: Rc<ShaderProgram>,
    pub render_pipeline: Box<dyn RenderPipeline>,
    pub widget: Box<dyn Widget>,
    pub camera: Camera
}

impl Widget2dScene {
    pub fn new(
        resource_manager: &mut ResourceManager,
        shader_paths: ShaderPathBundle,
        camera_bundle: CameraSize,
        render_pipeline: Box<dyn RenderPipeline>,
        widget: Box<dyn Widget>
    ) -> Result<Widget2dScene, EngineError> {
        let shader_program = resource_manager.load_shader_program(shader_paths)?;
        let camera = Camera::new(
            camera_bundle,
            CameraProjection::ORTHO,
            vec![&shader_program]
        )?;

        Ok(
            Widget2dScene {
                shader_program,
                render_pipeline,
                widget,
                camera
            }
        )
    }
}

impl Scene for Widget2dScene {
    fn set_size(&mut self, width: i32, height: i32) -> Result<(), EngineError> {
        self.render_pipeline.set_size(width, height)?;
        self.camera.width = width as f32;
        self.camera.height = height as f32;
        self.camera.send_proj()?;
        self.widget.set_size(width as f32, height as f32);

        self.set_widget_tree()?;

        Ok(())
    }

    fn draw(&mut self) -> Result<(), EngineError> {
        // Instanced rendering without depth testing draws in order of the matrices present,
        // resulting in everything being drawn in the correct order
        unsafe { gl::Disable(gl::DEPTH_TEST) };

        self.camera.send_view()?;
        self.camera.uniform_buffer
            .as_ref()
            .expect("Camera should be created with Camera::new()")
            .bind_ubo();

        self.render_pipeline.bind();
        self.shader_program.use_program();
        
        unsafe {
            gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, 0, self.widget_quad.dbo.get_id());
        }

        self.widget_quad.inner.draw(&self.shader_program)?;
        self.render_pipeline.draw()?;

        Ok(())
    }
}