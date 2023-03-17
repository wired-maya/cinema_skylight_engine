use std::rc::Rc;
use cgmath::{Quaternion, Matrix4, Vector3};

use crate::Model;

pub struct GameObject {
    pub position: Vector3<f32>,
    pub rotation: Quaternion<f32>,
    pub scale: f32,
    pub children: Vec<GameObject>,
    drawable: Option<Rc<Model>>,
    model_index: usize,
}

impl Default for GameObject {
    fn default() -> Self {
        Self {
            position: Vector3::<f32>::new(0.0, 0.0, 0.0),
            rotation: Quaternion::<f32>::new(1.0, 0.0, 0.0, 0.0),
            scale: 1.0,
            children: Default::default(),
            drawable: None,
            model_index: Default::default()
        }
    }
}

impl GameObject {
    pub fn from_model(model: Rc<Model>) -> GameObject {
        let mut obj = GameObject::default();
        obj.set_drawable(Some(model));

        obj
    }

    pub fn transform_matrix(&self) -> Matrix4<f32> {
        let mut matrix = Matrix4::<f32>::from_translation(self.position);
        matrix = matrix * Matrix4::<f32>::from_scale(self.scale);
        matrix = matrix * Matrix4::<f32>::from(self.rotation);

        matrix
    }

    // Needs to be called after changes are made to pos and rot
    pub fn set_transform_to_drawable(&mut self, vec_space: Matrix4<f32>) {
        let matrix = vec_space * self.transform_matrix();

        if let Some(drawable) = &self.drawable {
            drawable.borrow_mut().get_transform_array_mut().set_data_index(matrix, self.model_index);
        }

        for child in &mut self.children {
            child.set_transform_to_drawable(matrix);
        }
    }

    pub fn get_drawable(&self) -> &Option<Rc<Model>> {
        &self.drawable
    }

    pub fn set_drawable(&mut self, drawable: Option<Rc<Model>>) {
        // Remove old transform on model's transform buffer
        if let Some(drawable) = &self.drawable {
            drawable.borrow_mut().get_transform_array_mut().remove(self.model_index);
        }

        self.drawable = drawable;

        // Add new transform on model corresponding to this game object
        if let Some(drawable) = &self.drawable {
            self.model_index = drawable.borrow().get_transform_array().len();
            drawable.borrow_mut().get_transform_array_mut().push(self.transform_matrix());
        }
    }
}