use std::{rc::Rc, collections::HashMap, path::Path, cell::RefCell, fs::File, io::Read};
use cgmath::{vec3, vec2, Matrix4, SquareMatrix, Vector3, Vector2};
use silver_gl::{Model, Texture, Vertex, Mesh, GlImage, Skybox, ShaderProgram, ShaderCodeBundle};
use image::DynamicImage::*;

use crate::EngineError;

#[derive(Default)]
pub struct ResourceManager {
    model_store: HashMap<String, Rc<RefCell<Model>>>,
    texture_store: HashMap<String, Rc<Texture>>,
    shader_store: HashMap<ShaderPathBundle, Rc<ShaderProgram>>
}

// TODO: Make all loading async so that it is faster :)
// TODO: Time to beat: ~15 seconds on laptop
// TODO: Learn to use Rayon, Tokio
impl ResourceManager {
    fn _load_model(&mut self, path: &str) -> Result<Rc<RefCell<Model>>, EngineError> {
        let path = Path::new(path);
        let obj_path = path.to_str().unwrap().to_owned();
        let directory = path.parent().unwrap_or_else(|| Path::new("")).to_str().unwrap();
        
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
                    let texture = self.load_texture_2d(&format!("{}/{}", directory, &material.diffuse_texture))?;
                    gl_mesh.diffuse_textures.push(texture);
                } else {
                    gl_mesh.diffuse = vec3(material.diffuse[0], material.diffuse[1], material.diffuse[2]);
                }
                // Specular map
                if !material.specular_texture.is_empty() {
                    let texture = self.load_texture_2d(&format!("{}/{}", directory, &material.specular_texture))?;
                    gl_mesh.specular_textures.push(texture);
                } else {
                    gl_mesh.specular = vec3(material.specular[0], material.specular[1], material.specular[2]);
                }
                // Normal map
                if !material.normal_texture.is_empty() {
                    let texture = self.load_texture_2d(&format!("{}/{}", directory, &material.normal_texture))?;
                    gl_mesh.normal_textures.push(texture);
                }
                // Shininess map
                if !material.shininess_texture.is_empty() {
                    let texture = self.load_texture_2d(&format!("{}/{}", directory, &material.shininess_texture))?;
                    gl_mesh.shininess_textures.push(texture);
                } else {
                    gl_mesh.shininess = material.shininess; // Get all-mesh shininess if there is no map present
                }
            }

            meshes.push(gl_mesh);
        }

        // Init with identity matrix for now
        let model = Rc::new(RefCell::new(Model::new(
            vertices,
            indices,
            vec![Matrix4::identity()],
            meshes
        )));
        self.model_store.insert(obj_path, Rc::clone(&model));

        Ok(model)
    }

    pub fn load_model(&mut self, path: &str) -> Result<Rc<RefCell<Model>>, EngineError> {
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

    fn _load_texture_2d(&mut self, path: &str) -> Result<Rc<Texture>, EngineError> {
        let image = ResourceManager::load_image(path)?;
        let texture = Rc::new(Texture::from_2d(image));
        self.texture_store.insert(path.to_owned(), Rc::clone(&texture));

        Ok(texture)
    }

    pub fn load_texture_2d(&mut self, path: &str) -> Result<Rc<Texture>, EngineError> {
        if let Some(texture) = self.texture_store.get(path) {
            Ok(Rc::clone(texture))
        } else {
            self._load_texture_2d(path)
        }
    }

    fn _load_texture_cubemap(&mut self, path: &str) -> Result<Rc<Texture>, EngineError> {
        let image = ResourceManager::load_image(path)?;
        let texture = Rc::new(Texture::from_cubemap(image));
        self.texture_store.insert(path.to_owned(), Rc::clone(&texture));

        Ok(texture)
    }

    pub fn load_texture_cubemap(&mut self, path: &str) -> Result<Rc<Texture>, EngineError> {
        if let Some(texture) = self.texture_store.get(path) {
            Ok(Rc::clone(texture))
        } else {
            self._load_texture_cubemap(path)
        }
    }

    pub fn load_skybox(&mut self, path: &str) -> Result<Skybox, EngineError> {
        // Cube definition
        let vertices = vec![
            Vertex {
                position: Vector3::new(-1.0, 1.0, -1.0),
                normal: Vector3::new(0.0, 0.0, 0.0),
                tex_coord: Vector2::new(0.0, 0.0),
                ..Vertex::default()
            },
            Vertex {
                position: Vector3::new(1.0, 1.0, -1.0),
                normal: Vector3::new(0.0, 0.0, 0.0),
                tex_coord: Vector2::new(0.0, 0.0),
                ..Vertex::default()
            },
            Vertex {
                position: Vector3::new(-1.0, -1.0, -1.0),
                normal: Vector3::new(0.0, 0.0, 0.0),
                tex_coord: Vector2::new(0.0, 0.0),
                ..Vertex::default()
            },
            Vertex {
                position: Vector3::new(1.0, -1.0, -1.0),
                normal: Vector3::new(0.0, 0.0, 0.0),
                tex_coord: Vector2::new(0.0, 0.0),
                ..Vertex::default()
            },
            Vertex {
                position: Vector3::new(-1.0, 1.0, 1.0),
                normal: Vector3::new(0.0, 0.0, 0.0),
                tex_coord: Vector2::new(0.0, 0.0),
                ..Vertex::default()
            },
            Vertex {
                position: Vector3::new(1.0, 1.0, 1.0),
                normal: Vector3::new(0.0, 0.0, 0.0),
                tex_coord: Vector2::new(0.0, 0.0),
                ..Vertex::default()
            },
            Vertex {
                position: Vector3::new(-1.0, -1.0, 1.0),
                normal: Vector3::new(0.0, 0.0, 0.0),
                tex_coord: Vector2::new(0.0, 0.0),
                ..Vertex::default()
            },
            Vertex {
                position: Vector3::new(1.0, -1.0, 1.0),
                normal: Vector3::new(0.0, 0.0, 0.0),
                tex_coord: Vector2::new(0.0, 0.0),
                ..Vertex::default()
            },
        ];

        let indices = vec![
            0, 2, 3, 3, 1, 0,
            6, 2, 0, 0, 4, 6,
            3, 7, 5, 5, 1, 3,
            6, 4, 5, 5, 7, 6,
            0, 1, 5, 5, 4, 0,
            2, 6, 3, 3, 6, 7
        ];

        let model_transforms = vec![Matrix4::<f32>::from_translation(vec3(0.0, 0.0, 0.0))];

        let mut model = Model::new(
            vertices,
            indices,
            model_transforms,
            vec![Mesh::new(0, 36)]
        );
        model.meshes[0].diffuse_textures.push(self.load_texture_cubemap(path)?);

        Ok(Skybox { model })
    }

    fn load_shader(&mut self, path: &str) -> Result<String, EngineError> {
        let mut shader_file = File::open(path)?;
        let mut shader_code = String::new();

        shader_file.read_to_string(&mut shader_code)?;

        Ok(shader_code)
    }

    fn load_shaders(&mut self, paths: ShaderPathBundle) -> Result<Rc<ShaderProgram>, EngineError> {
        let mut code_bundle = ShaderCodeBundle::default();

        if let Some(vert) = &paths.vertex {
            code_bundle.vertex = Some(self.load_shader(vert)?);
        }
        if let Some(geom) = &paths.geometry {
            code_bundle.geometry = Some(self.load_shader(geom)?);
        }
        if let Some(frag) = &paths.fragment {
            code_bundle.fragment = Some(self.load_shader(frag)?);
        }

        let shader_program = Rc::new(ShaderProgram::new(code_bundle)?);

        self.shader_store.insert(paths, Rc::clone(&shader_program));
        Ok(shader_program)
    }

    pub fn load_shader_program(&mut self, paths: ShaderPathBundle) -> Result<Rc<ShaderProgram>, EngineError> {
        if let Some(shader) = self.shader_store.get(&paths) {
            Ok(Rc::clone(shader))
        } else {
            self.load_shaders(paths)
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
pub struct ShaderPathBundle {
    pub vertex: Option<String>,
    pub geometry: Option<String>,
    pub fragment: Option<String>
}