use std::rc::Rc;
use silver_gl::{Model, Texture};

pub struct ResourceManager {
    model_store: Vec<Rc<Model>>,
    texture_store: Vec<Rc<Texture>>
}

impl ResourceManager {
    fn load_model(&mut self, path: &str) -> Rc<Model> {
        todo!()
    }
}