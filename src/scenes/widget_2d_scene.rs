use std::{rc::Rc};
use cgmath::{Vector4, Matrix4, SquareMatrix};
use silver_gl::{RenderPipeline, Model, ShaderProgram};
use crate::{Camera, EngineError, ResourceManager, ShaderPathBundle, CameraSize, create_wquad, BackgroundWidget, Widget, Scene, CameraProjection};

pub struct Widget2dScene {
    pub widget_quad: Model,
    pub widget_shader_program: Rc<ShaderProgram>,
    pub render_pipeline: Box<dyn RenderPipeline>,
    // TODO: Maybe make this an array?
    pub top_widget: BackgroundWidget, // TODO: Should become widget box (also rename to child)
    pub camera: Camera,
    
}

impl Widget2dScene {
    pub fn new(
        resource_manager: &mut ResourceManager,
        widget_shader_paths: ShaderPathBundle,
        camera_bundle: CameraSize,
        render_pipeline: Box<dyn RenderPipeline>,
        bottom_colour: Vector4<f32>
    ) -> Result<Widget2dScene, EngineError> {
        let widget_shader_program = resource_manager.load_shader_program(widget_shader_paths)?;
        // TODO: CameraSize should be taken by Camera::new
        let mut camera = Camera::new(
            camera_bundle.width,
            camera_bundle.height,
            camera_bundle.fov,
            vec![&widget_shader_program]
        )?;

        // TODO: Move this to camera bundle
        camera.projection = CameraProjection::ORTHO;
        camera.send_proj()?;

        Ok(
            Widget2dScene {
                widget_quad: create_wquad(),
                widget_shader_program,
                render_pipeline,
                top_widget: BackgroundWidget {
                    colour: bottom_colour,
                    width: camera_bundle.width as f32,
                    height: camera_bundle.height as f32,
                    ..Default::default()
                },
                camera
            }
        )
    }

    // All in one functions to simplify the recusive widget-specific function
    pub fn set_widget_tree(&mut self) -> Result<(), EngineError> {
        // Clear all quad props
        self.widget_quad.meshes[0].diffuse_textures.clear();
        unsafe { self.widget_quad.tbo.clear_inner() };

        // Recursively set all widget info
        self.widget_shader_program.use_program();
        self.top_widget.traverse_and_push_all(&mut self.widget_quad, &self.widget_shader_program, Matrix4::identity())?;

        // Finally send the batched transforms
        self.widget_quad.tbo.send_data_mut();

        Ok(())
    }

    pub fn set_widget_transforms(&mut self) -> Result<(), EngineError> {
        // TODO: This needs to follow vec space
        self.top_widget.traverse_and_set_transforms(&mut self.widget_quad, Matrix4::identity())?;
        self.widget_quad.tbo.send_data_mut();

        Ok(())
    }

    pub fn set_widget_textures(&mut self) -> Result<(), EngineError> {
        self.widget_quad.meshes[0].diffuse_textures.clear();
        self.top_widget.traverse_and_set_textures(&mut self.widget_quad)?;

        Ok(())
    }

    pub fn send_widget_info(&self) -> Result<(), EngineError> {
        self.widget_shader_program.use_program();
        self.top_widget.traverse_and_send_info(&self.widget_shader_program)?;

        Ok(())
    }
}

impl Scene for Widget2dScene {
    fn set_size(&mut self, width: i32, height: i32) -> Result<(), EngineError> {
        self.render_pipeline.set_size(width, height)?;
        self.camera.width = width as f32;
        self.camera.height = height as f32;
        self.camera.send_proj()?;
        self.top_widget.width = width as f32;
        self.top_widget.height = height as f32;

        self.set_widget_tree()?;

        Ok(())
    }

    fn draw(&mut self) -> Result<(), EngineError> {
        // Instanced rendering without depth testing draws in order of the matrices present,
        // resulting in everything being drawn in the correct order
        unsafe { gl::Disable(gl::DEPTH_TEST) };

        self.camera.send_view()?;

        self.render_pipeline.bind();
        self.widget_shader_program.use_program();

        self.send_widget_info()?;
        self.widget_quad.draw(&self.widget_shader_program)?;

        self.render_pipeline.draw()?;

        Ok(())
    }
}