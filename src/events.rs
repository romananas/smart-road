use sdl2::{rect::Point, EventPump};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub enum Type {
    SpawnCar(Vec<Point>),
    Quit,
    None,
}

pub fn handle(ep: &mut EventPump) -> Type {
    for event in ep.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => return Type::Quit,
            _ => return Type::None,
        }
    };
    Type::None
}