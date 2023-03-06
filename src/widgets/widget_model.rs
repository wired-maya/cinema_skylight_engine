use cgmath::Matrix4;
use memoffset::offset_of;
use silver_gl::{Vertex, GlError, VertexArray, Buffer, gl};

// Modified model struct from silver_gl to allow for greater compatibility
// witht he widget system.
pub struct WModel {
    pub vao: VertexArray,
    pub vbo: Buffer<Vertex>,
    pub ebo: Buffer<u32>,
    pub tbo: Buffer<Matrix4<f32>>,
    pub dbo: Buffer<u8>
}

impl WModel {
    // Will need to call Model::calc_vertex_tangents on verices and indices
    // if you plan on using something more advanced than quadrilaterals
    pub fn new(
        vertices: Vec<Vertex>,
        indices: Vec<u32>,
        model_transforms: Vec<Matrix4<f32>>,
        data_buffer: Vec<u8>
    ) -> WModel {
        let mut model = WModel {
            vao: VertexArray::new(),
            vbo: Buffer::new(),
            ebo: Buffer::new(),
            tbo: Buffer::new(),
            dbo: Buffer::new()
        };

        model.setup_model(vertices, indices);
        model.setup_transform_attribute(model_transforms);
        model.dbo.set_data_mut(data_buffer);

        model
    }

    pub fn draw(&self) -> Result<(), GlError> {
        unsafe {
            self.vao.bind();

            self.vao.draw_elements(
                self.ebo.len() as i32,
                self.tbo.len() as i32
            );

            gl::BindVertexArray(0);
        }

        Ok(())
    }

    pub fn setup_model(&mut self, vertices: Vec<Vertex>, indices: Vec<u32>) {
        self.vao.add_vertex_buffer(&mut self.vbo);
        self.vao.set_element_buffer(&mut self.ebo);

        self.vao.add_attrib(&mut self.vbo, 3, offset_of!(Vertex, position) as u32, gl::FLOAT);
        self.vao.add_attrib(&mut self.vbo, 3, offset_of!(Vertex, normal) as u32, gl::FLOAT);
        self.vao.add_attrib(&mut self.vbo, 2, offset_of!(Vertex, tex_coord) as u32, gl::FLOAT);
        self.vao.add_attrib(&mut self.vbo, 3, offset_of!(Vertex, tangent) as u32, gl::FLOAT);
        self.vao.add_attrib(&mut self.vbo, 3, offset_of!(Vertex, bitangent) as u32, gl::FLOAT);

        self.vbo.set_data(vertices);
        self.ebo.set_data(indices);
    }
    
    pub fn setup_transform_attribute(&mut self, model_transforms: Vec<Matrix4<f32>>) {
        self.vao.add_vertex_buffer(&mut self.tbo);
        self.vao.add_attrib_divisor(&mut self.tbo, 4);
        self.tbo.set_data_mut(model_transforms);
    }
}