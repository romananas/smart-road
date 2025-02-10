extern crate sdl2;

mod assets;
mod events;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use smart_road::display::Display;
use std::time::Duration;

use smart_road::cars::*;

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
    let mut car = Car::new(Point::new(550, 550), 20, 40);
    car.show_collisions(true);
    car.show_detections(true);
    
    'running: loop {
        match events::handle(&mut event_pump) {
            events::Type::Quit => break 'running,
            _ => {},
        }

        canvas.clear();

        assets::load_map(&mut canvas)?;

        //car.go_to(Point::new(1100, 550));

        let _ = car.display(&mut canvas);

        canvas.present();

        // std::thread::sleep(Duration::from_millis(16));
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 160));
    }

    Ok(())
}