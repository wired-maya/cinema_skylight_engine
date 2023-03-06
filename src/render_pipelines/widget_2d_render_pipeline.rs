use std::rc::Rc;

use silver_gl::{Framebuffer, GlError, RenderPipeline, Texture, gl};

use crate::EngineError;

pub struct Widget2dRenderPipeline {
    intermediate_fb: Framebuffer,
    width: i32,
    height: i32
}

impl Widget2dRenderPipeline {
    pub fn new(
        width: i32,
        height: i32
    ) -> Result<Widget2dRenderPipeline, EngineError> {
        let intermediate_fb = Framebuffer::new(
            width,
            height,
            1,
            true
        )?;

        Ok(
            Widget2dRenderPipeline {
                intermediate_fb,
                width,
                height
            }
        )
    }
}

impl RenderPipeline for Widget2dRenderPipeline {
    fn bind(&self) {
        unsafe {
            gl::Viewport(0, 0, self.width, self.height);
            self.intermediate_fb.bind();
            gl::Clear(gl::DEPTH_BUFFER_BIT);
        }
    }

    // No post processing is done (yet), so nothing needed here
    fn draw(&mut self) -> Result<(), GlError> {
        Ok(())
    }

    fn get_height(&self) -> (i32, i32) {
        (self.width, self.height)
    }

    fn set_size(&mut self, width: i32, height: i32) -> Result<(), GlError> {
        self.width = width;
        self.height = height;
        self.intermediate_fb.set_size(width, height)?;

        Ok(())
    }

    fn get_link(&self) -> Result<Vec<Rc<Texture>>, GlError> {
        Ok(self.intermediate_fb.get_link())
    }

    fn link_to(&mut self, output: Vec<Rc<Texture>>) -> Result<(), GlError> {
        for texture in output {
            self.intermediate_fb.link_push(texture);
        }

        Ok(())
    }

    fn link_push(&mut self, texture: Rc<Texture>) -> Result<(), GlError> {
        self.intermediate_fb.link_push(texture);
        
        Ok(())
    }

    fn unlink(&mut self) {
        self.intermediate_fb.unlink();
    }
}