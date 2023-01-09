use std::{rc::Rc, collections::HashMap, path::Path};
use cgmath::{vec3, vec2, Matrix4, SquareMatrix};
use silver_gl::{Model, Texture, Vertex, Mesh, GlImage};
use image::DynamicImage::*;

use crate::EngineError;

pub struct ResourceManager {
    model_store: HashMap<String, Rc<Model>>,
    texture_store: HashMap<String, Rc<Texture>>
}

impl ResourceManager {
    fn _load_model(&mut self, path: &str) -> Result<Rc<Model>, EngineError> {
        let path = Path::new(path);
        let obj_path = path.to_str().unwrap().to_owned();
        let directory = path.parent().unwrap_or_else(|| Path::new("")).to_str().unwrap().to_owned();
        
        let obj = tobj::load_obj(path, &tobj::GPU_LOAD_OPTIONS);

        let (models, materials) = obj?;
        let materials = materials?;

        // Combine all meshes for optimized rendering
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();
        let mut meshes: Vec<Mesh> = Vec::new();

        for model in models {
            let mesh = &model.mesh;
            let num_vertices = mesh.positions.len() / 3;

            // Push to model vertices
            let (p, n, t) = (&mesh.positions, &mesh.normals, &mesh.texcoords);
            for i in 0..num_vertices {
                vertices.push(
                    Vertex {
                        position: vec3(p[i*3], p[i*3+1], p[i*3+2]),
                        normal: vec3(n[i*3], n[i*3+1], n[i*3+2]),
                        tex_coord: vec2(t[i*2], t[i*2+1]),
                        ..Vertex::default()
                    }
                )
            }

            // Push to model indices while adjusting for offset
            let offset = indices.len();
            let mut adjusted_indices: Vec<u32> = mesh.indices.iter().map(|index| { index + offset as u32 }).collect();
            indices.append(&mut adjusted_indices);

            // Process material
            let mut gl_mesh = Mesh::new(offset, mesh.indices.len() as i32);
            if let Some(material_id) = mesh.material_id {
                let material = &materials[material_id];

                // Diffuse map
                if !material.diffuse_texture.is_empty() {
                    // TODO: concat with directory!
                    let texture = self.load_texture_2D(&material.diffuse_texture)?;
                    gl_mesh.diffuse_textures.push(texture);
                } else {
                    gl_mesh.diffuse = vec3(material.diffuse[0], material.diffuse[1], material.diffuse[2]);
                }
                // Specular map
                if !material.specular_texture.is_empty() {
                    let texture = self.load_texture_2D(&material.specular_texture)?;
                    gl_mesh.specular_textures.push(texture);
                } else {
                    gl_mesh.specular = vec3(material.specular[0], material.specular[1], material.specular[2]);
                }
                // Normal map
                if !material.normal_texture.is_empty() {
                    let texture = self.load_texture_2D(&material.normal_texture)?;
                    gl_mesh.normal_textures.push(texture);
                }
                // Shininess map
                if !material.shininess_texture.is_empty() {
                    let texture = self.load_texture_2D(&material.shininess_texture)?;
                    gl_mesh.shininess_textures.push(texture);
                } else {
                    gl_mesh.shininess = material.shininess; // Get all-mesh shininess if there is no map present
                }
            }

            meshes.push(gl_mesh);
        }

        // Init with identity matrix for now
        let model = Rc::new(Model::new(vertices, indices, vec![Matrix4::identity()]));
        self.model_store.insert(obj_path, Rc::clone(&model));

        Ok(model)
    }

    pub fn load_model(&mut self, path: &str) -> Result<Rc<Model>, EngineError> {
        if let Some(model) = self.model_store.get(path) {
            Ok(Rc::clone(model))
        } else {
            self._load_model(path)
        }
    }

    fn load_image(path: &str) -> Result<GlImage, EngineError> {
        let img = image::io::Reader::open(path)?.decode()?;

        // TODO: if there is an alpha, mark texture as transparent
        let (internal_format, data_format) = match img {
            ImageLuma8(_) => (gl::RED, gl::RED),
            ImageLumaA8(_) => (gl::RG, gl::RG),
            ImageRgb8(_) => (gl::SRGB, gl::RGB),
            ImageRgba8(_) => (gl::SRGB_ALPHA, gl::RGBA),
            _ => (gl::SRGB, gl::RGB) // If nothing else, try default
        };

        Ok(
            GlImage {
                bytes: Vec::from(img.as_bytes()),
                internal_format,
                data_format,
                width: img.width() as i32,
                height: img.height() as i32
            }
        )
    }

    fn _load_texture_2D(&mut self, path: &str) -> Result<Rc<Texture>, EngineError> {
        let image = ResourceManager::load_image(path)?;
        let texture = Rc::new(Texture::from_2D(image));
        self.texture_store.insert(path.to_owned(), Rc::clone(&texture));

        Ok(texture)
    }

    pub fn load_texture_2D(&mut self, path: &str) -> Result<Rc<Texture>, EngineError> {
        if let Some(texture) = self.texture_store.get(path) {
            Ok(Rc::clone(texture))
        } else {
            self._load_texture_2D(path)
        }
    }

    // TODO: Make cube map loader, which takes in one image of squares arranged on one
    // TODO: texture in a tideways t (what shows up when you google cube map), then use
    // TODO: height / 3 to get the square's dimentions, and write from an offset in the
    // TODO: image (hopefully this works)

    // TODO: Make a shader program loader that takes in a ShaderBundle with a bunch of
    // TODO: optional paths for the shader program, implement an eq trait, have that as
    // TODO: a key in the shader store
}