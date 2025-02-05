extern crate sdl2;

use sdl2::{pixels::Color, rect::Point};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use smart_road::cars::cars::Rotation;
use smart_road::display::Display;
use std::time::Duration;

use smart_road::cars::Car;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 800, 800)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut car_a: Car = Car::new(Point::new(400, 400), 20, 30);
    car_a.show_collisions(true);
    car_a.show_detections(true);

    let mut car_b: Car = Car::new(Point::new(400, 400), 20, 30);
    car_b.show_collisions(true);
    car_b.show_detections(true);


    'running: loop {
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::R),.. } => {
                    car_a.rotate(Rotation::Right);
                },
                _ => {}
            }
        }
        // Récupérer la position du curseur
        let mouse_state = event_pump.mouse_state();
        let x = mouse_state.x();
        let y = mouse_state.y();

        car_a.move_to(x, y);

        
        let _ = car_a.state_check(&car_b);

        let _ = car_a.display(&mut canvas);

        let _ = car_b.display(&mut canvas);


        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}