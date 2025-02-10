use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::cars;

pub enum Type {
    SpawnCar((cars::Direction,cars::Direction)), // direction : from,to
    Quit, // to quit the program
    None, // nothing happen
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