use std::{rc::Rc, cell::RefCell};
use cgmath::{Matrix4, SquareMatrix};
use silver_gl::{Model, Skybox, ShaderProgram, RenderPipeline, gl};
use crate::{Camera, GameObject, CameraSize, ShaderPathBundle, ResourceManager, EngineError, Scene};

// TODO: add lights, need a light trait
// TODO: See if qsort is fast enough that  to allow me to sort models based on distance from the camera every frame, enabling transparency
pub struct View3DScene {
    pub models: Vec<Rc<RefCell<Model>>>,
    pub model_shader_program: Rc<ShaderProgram>,
    pub skybox: Skybox,
    pub skybox_shader_program: Rc<ShaderProgram>,
    pub camera: Camera,
    pub render_pipeline: Box<dyn RenderPipeline>,
    pub world_obj: GameObject
}

impl View3DScene {
    pub fn new(
        resource_manager: &mut ResourceManager,
        model_shader_paths: ShaderPathBundle,
        skybox_path: &str,
        skybox_shader_paths: ShaderPathBundle,
        camera_bundle: CameraSize,
        render_pipeline: Box<dyn RenderPipeline>
    ) -> Result<View3DScene, EngineError> {
        let model_shader_program = resource_manager.load_shader_program(model_shader_paths)?;
        let skybox_shader_program = resource_manager.load_shader_program(skybox_shader_paths)?;

        let skybox = resource_manager.load_skybox(skybox_path)?;

        let camera = Camera::new(
            camera_bundle,
            crate::CameraProjection::PERSPECTIVE,
            vec![&model_shader_program, &skybox_shader_program]
        )?;
        
        Ok(
            View3DScene {
                models: vec![],
                model_shader_program,
                skybox,
                skybox_shader_program,
                camera,
                render_pipeline,
                world_obj: GameObject::default()
            }
        )
    }
}

impl Scene for View3DScene {
    fn set_size(&mut self, width: i32, height: i32) -> Result<(), EngineError> {
        self.render_pipeline.set_size(width, height)?;
        self.camera.width = width as f32;
        self.camera.height = height as f32;
        self.camera.send_proj()?;

        Ok(())
    }

    fn draw(&mut self) -> Result<(), EngineError> {
        unsafe { gl::Enable(gl::DEPTH_TEST) };

        self.camera.send_view()?;
        self.camera.uniform_buffer
            .as_ref()
            .expect("Camera should be instantiated with Camera::new()")
            .bind_ubo();

        self.render_pipeline.bind();
        self.model_shader_program.use_program();

        self.world_obj.set_transform_to_drawable(Matrix4::<f32>::identity());
        
        // TODO: Find a way to have instanced rendering only render a certain subset of
        // TODO: the transforms for this scene, so that multiple 3D scenes with different
        // TODO: shaders can use the same models
        for model in self.models.iter() {
            model.borrow().draw(&self.model_shader_program)?;
        }

        // Drawn last so it only is drawn over unused pixels, improving performance
        self.skybox.draw(&self.skybox_shader_program)?;

        self.render_pipeline.draw()?;

        Ok(())
    }
}