pub type Widget = Box<dyn Drawable + Send>;

pub trait Drawable {
    fn draw(&self);
    fn load_assets(&mut self);
}