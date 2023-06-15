use cgmath::{Vector3, Vector2};
use silver_gl::{Vertex, MultiBindModel, ModelCreateTrait, Mesh};

pub fn create_wquad() -> MultiBindModel {
    // Flat panel definition
    // Starts at (0.0, 0.0) for a default top-left
    // anchor
    let vertices = vec![
        Vertex {
            position: Vector3::new(0.0, 0.0, 0.0),
            normal: Vector3::new(0.0, 0.0, 1.0),
            tex_coord: Vector2::new(0.0, 1.0),
            ..Vertex::default()
        },
        Vertex {
            position: Vector3::new(0.0, 1.0, 0.0),
            normal: Vector3::new(0.0, 0.0, 1.0),
            tex_coord: Vector2::new(0.0, 0.0),
            ..Vertex::default()
        },
        Vertex {
            position: Vector3::new(1.0, 1.0, 0.0),
            normal: Vector3::new(0.0, 0.0, 1.0),
            tex_coord: Vector2::new(1.0, 0.0),
            ..Vertex::default()
        },
        Vertex {
            position: Vector3::new(1.0, 0.0, 0.0),
            normal: Vector3::new(0.0, 0.0, 1.0),
            tex_coord: Vector2::new(1.0, 1.0),
            ..Vertex::default()
        }
    ];

    let indices = vec![
        0, 1, 2,
        0, 2, 3
    ];

    MultiBindModel::new(
        vertices,
        indices,
        Vec::new(),
        vec![Mesh::new(0, 6)]
    )
}