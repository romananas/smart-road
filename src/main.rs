extern crate sdl2;

mod map;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

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
    
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.clear();

        map::load_map(&mut canvas)?;

        canvas.present();

        std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}