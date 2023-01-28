use crate::EngineError;

pub trait Scene {
    fn set_size(&mut self, width: i32, height: i32) -> Result<(), EngineError>;
    fn draw(&mut self) -> Result<(), EngineError>;
}