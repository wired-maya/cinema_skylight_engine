use cgmath::Matrix4;
use silver_gl::{Vertex, Buffer, ModelTrait};

use crate::{ResourceManager, EngineError};

// TODO: Wrapper over silver_gl model to allow for easier compatibility
// TODO: with the widget system
pub struct WModel {
    pub inner: Box<dyn ModelTrait>,
    pub dbo: Buffer<u8>
}

impl WModel {
    // Will need to call Model::calc_vertex_tangents on verices and indices
    // if you plan on using something more advanced than quadrilaterals
    pub fn new(
        resource_manager: &ResourceManager,
        vertices: Vec<Vertex>,
        indices: Vec<u32>,
        model_transforms: Vec<Matrix4<f32>>,
        data_buffer: Vec<u8>
    ) -> Result<WModel, EngineError> {
        let mut model = WModel {
            inner: resource_manager.create_model(vertices, indices, model_transforms, vec![])?,
            dbo: Buffer::new()
        };

        model.dbo.set_data_mut(data_buffer);

        Ok(model)
    }
}