use sdl2::render::Canvas;
use sdl2::video::Window;
pub trait Display {
    type Error;
    fn display(&self,canvas: &mut Canvas<Window>) -> Self::Error; 
}