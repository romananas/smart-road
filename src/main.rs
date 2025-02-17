mod entities;
mod events;
mod cars;
mod map;

use std::time::Duration;
use cars::UpdateState;
use sdl2::{pixels::Color, video::Window};

use entities::Entity;

const SCREEN_SIZE: (u32,u32) = (1100,1100);
const COOLDOWN_MS: u64 = 600;
const DEBUG: bool = true;

fn init_window(sdl_context: sdl2::Sdl) -> Result<Window,String> {
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Affichage d'une partie de l'image", SCREEN_SIZE.0, SCREEN_SIZE.1)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    Ok(window)
}

fn main() -> Result<(), String> {
    // let mut ctrl = entities::Controller::init();

    let sdl_context = sdl2::init()?;
    let window = init_window(sdl_context.clone())?;
    let mut canvas = window.into_canvas().present_vsync().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;

    let mut cars: Vec<cars::Car> = Vec::new();

    let mut now = std::time::Instant::now();

    let mut car_spawned = 0u32;
    let mut car_passed: u32 = 0u32;

    let mut collisions_count: u32 = 0u32;

    'running: loop {
        match events::handle(&mut event_pump)
        {
            events::Type::SpawnCar(from,to) => {
                if now.elapsed() >= Duration::from_millis(COOLDOWN_MS) {
                    cars.push(map::spawn_car(from, to, 32,45).unwrap());
                    car_spawned += 1;
                    now = std::time::Instant::now();
                }
            },
            events::Type::Quit => break 'running,
            _ => {},
        };

        canvas.set_draw_color(sdl2::pixels::Color::BLACK);
        canvas.clear();

        map::show_points(&mut canvas)?;

        let cars_clone = cars.clone();
        let mut to_remove = -1;
        for (i,c) in cars.iter_mut().enumerate() {
            let mut tmp = cars_clone.clone();
            tmp.remove(i);
            match  c.update(tmp) {
                UpdateState::Finished => to_remove = i as i32,
                UpdateState::Waiting => {
                    let r = match (DEBUG,c.get_detections()) {
                        (true,Some((l,u))) => (l,u),
                        _ => break,
                    };
                    canvas.set_draw_color(Color::RED);
                    canvas.draw_rect(r.0).unwrap();
                    canvas.draw_rect(r.1).unwrap();
                },
                UpdateState::Collided => {
                    collisions_count += 1;
                    to_remove = i as i32;
                },
                _ => {
                    let r = match (DEBUG,c.get_detections()) {
                        (true,Some((l,u))) => (l,u),
                        _ => break,
                    };
                    canvas.set_draw_color(Color::GRAY);
                    canvas.draw_rect(r.0).unwrap();
                    canvas.draw_rect(r.1).unwrap();
                },
            }
            c.display(&mut canvas).unwrap();
        }
        if to_remove != -1 {
            car_passed += 1;
            cars.remove(to_remove as usize);
        }

        canvas.present();
        std::thread::sleep(Duration::from_nanos(1_000_000_000u64 / 60));
    }

    println!("\ncar spawned : {}\ncar passed  : {}\ncollisions  : {}\n",car_passed,car_spawned,collisions_count);
    Ok(())
}
