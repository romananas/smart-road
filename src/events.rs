use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use rand::prelude::*;
use smart_road::cars::cars::Direction;

use crate::cars;

#[allow(dead_code)]
pub enum Type {
    SpawnCar(cars::Direction,cars::Direction), // direction : from,to
    Quit, // to quit the program
    None, // nothing happen
}

fn gen_rand_direction() -> cars::Direction {
    let mut rng = rand::rng();
    match rng.random_range(0u8..3u8) {
        0 => Direction::North,
        1 => Direction::South,
        2 => Direction::East,
        3 => Direction::West,
        _ => panic!("error in gen rand direction"),
    }
    
}

pub fn handle(ep: &mut EventPump) -> Type {
    for event in ep.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::ESCAPE),
                ..
            } => return Type::Quit,
            Event::KeyDown { keycode: Some(Keycode::UP), .. } => return Type::SpawnCar(Direction::North,gen_rand_direction()),
            Event::KeyDown { keycode: Some(Keycode::DOWN), .. } => return Type::SpawnCar(Direction::South,gen_rand_direction()),
            Event::KeyDown { keycode: Some(Keycode::RIGHT), .. } => return Type::SpawnCar(Direction::East,gen_rand_direction()),
            Event::KeyDown { keycode: Some(Keycode::LEFT), .. } =>return Type::SpawnCar(Direction::West,gen_rand_direction()),
            Event::KeyDown { keycode: Some(Keycode::R), .. } =>return Type::SpawnCar(gen_rand_direction(),gen_rand_direction()),
            _ => return Type::None,
        }
    };
    Type::None
}