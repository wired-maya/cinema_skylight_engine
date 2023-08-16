use crate::EngineError;

pub trait Scene {
    fn get_size(&self) -> (i32, i32);
    fn set_size(&mut self, width: i32, height: i32) -> Result<(), EngineError>;

    fn draw(&mut self) -> Result<(), EngineError>;
}