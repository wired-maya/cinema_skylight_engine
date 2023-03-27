use std::{rc::Rc};
use cgmath::{Matrix4, SquareMatrix};
use silver_gl::{RenderPipeline, ShaderProgram, gl};
use crate::{Camera, EngineError, ResourceManager, ShaderPathBundle, CameraSize, create_wquad, Widget, Scene, CameraProjection, widget_model::WModel};

pub struct Widget2dScene {
    pub widget_quad: WModel,
    pub widget_shader_program: Rc<ShaderProgram>,
    pub render_pipeline: Box<dyn RenderPipeline>,
    pub widget: Box<dyn Widget>,
    pub camera: Camera
}

impl Widget2dScene {
    pub fn new(
        resource_manager: &mut ResourceManager,
        widget_shader_paths: ShaderPathBundle,
        camera_bundle: CameraSize,
        render_pipeline: Box<dyn RenderPipeline>,
        widget: Box<dyn Widget>
    ) -> Result<Widget2dScene, EngineError> {
        let widget_shader_program = resource_manager.load_shader_program(widget_shader_paths)?;
        let camera = Camera::new(
            camera_bundle,
            CameraProjection::ORTHO,
            vec![&widget_shader_program]
        )?;

        Ok(
            Widget2dScene {
                widget_quad: create_wquad(resource_manager)?,
                widget_shader_program,
                render_pipeline,
                widget,
                camera
            }
        )
    }

    // All in one functions to simplify the recusive widget-specific function
    pub fn set_widget_tree(&mut self) -> Result<(), EngineError> {
        // Clear all quad props
        unsafe {
            self.widget_quad.inner.get_transform_array_mut().clear_inner();
            self.widget_quad.dbo.clear_inner();
            self.widget_quad.inner.get_meshes_mut().clear();
        };

        // Recursively set all widget info
        self.widget_shader_program.use_program();
        self.widget.traverse_and_push_all(&mut self.widget_quad, &self.widget_shader_program, Matrix4::identity())?;

        // Finally send the batched transforms
        self.widget_quad.inner.get_transform_array().send_data_mut();
        self.widget_quad.dbo.send_data_mut();

        Ok(())
    }

    pub fn set_widget_transforms(&mut self) -> Result<(), EngineError> {
        self.widget.traverse_and_set_transforms(&mut self.widget_quad, Matrix4::identity())?;
        self.widget_quad.inner.get_transform_array().send_data_mut();

        Ok(())
    }

    // Requires shader program use
    pub fn send_widget_info(&mut self) -> Result<(), EngineError> {
        self.widget.traverse_and_send_info(&mut self.widget_quad)?;

        Ok(())
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
            .expect("Camera should be instantiated with Camera::new()")
            .bind_ubo();

        self.render_pipeline.bind();
        self.widget_shader_program.use_program();
        
        unsafe {
            gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, 0, self.widget_quad.dbo.get_id());
        }

        self.widget_quad.inner.draw(&self.widget_shader_program)?;
        self.render_pipeline.draw()?;

        Ok(())
    }
}