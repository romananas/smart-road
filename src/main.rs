extern crate sdl2;

// mod map;
mod events;
mod spwn;
// mod debug;

use sdl2::{image::LoadTexture, rect::Point};
use std::time::Duration;

use smart_road::{cars::*, display::Display};

const CAR_DEF_WIDTH: u32= 20;
const CAR_DEF_LENGHT: u32= 40;

fn logic(car: &mut cars::Car,others: Vec<cars::Car>) {
    for other in others {
        if car.has_intersection(&other) == Some(cars::IntersectionType::SawCollisionBox) {
            car.velocity = other.velocity;
            return;
        }
        if car.has_intersection(&other) == Some(cars::IntersectionType::SawDetectionBox) && car.cmp(other) == cars::Cmp::Crossing {
            use cars::Direction::*;
            match (car.get_direction(),other.get_direction()) {
                (North,West) | (South,East) | (East,North) | (West,South) => car.velocity = other.velocity/ 2,
                _ => {},
            }
        }
    }
    car.velocity = cars::BASE_VELOCITY;
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Affichage d'une partie de l'image", 1100, 1100)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().present_vsync().build().map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    let texture_creator = canvas.texture_creator();
    let background = texture_creator.load_texture("assets/road.png")?;

    let mut cars: Vec<(Car,Vec<Point>)> = Vec::new();
    
    'running: loop {
        use events::Type;
        match events::handle(&mut event_pump)
        {
            Type::Quit => break 'running,
            Type::SpawnCar(from,to) => {
                cars.push(spwn::spwn(from, to, CAR_DEF_WIDTH, CAR_DEF_LENGHT).unwrap());
            }
            _ => {},
        };

        canvas.set_draw_color(sdl2::pixels::Color::BLACK);
        canvas.clear();

        canvas.copy(&background, None, None)?;

        cars.retain_mut(|(car, path)| !car.follow(path));
        let cloned_cars = cars.iter().map(|(c,_)| c.clone()).collect::<Vec<Car>>();
        for (i, cp) in cars.iter_mut().enumerate() {
            let mut tmp = cloned_cars.clone();
            tmp.remove(i);
            logic(&mut cp.0, tmp);

        }
        cars.iter().for_each(|(c,_)| {let _ = c.display(&mut canvas);});

        canvas.present();

        // std::thread::sleep(Duration::from_millis(16));
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 160));
    }

    Ok(())
}