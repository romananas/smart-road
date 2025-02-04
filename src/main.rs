extern crate sdl2;

use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use std::time::Duration;

fn main() -> Result<(), String> {
    // Initialisation de SDL2
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    // Création de la fenêtre
    let window = video_subsystem
        .window("Affichage d'une partie de l'image", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().present_vsync().build().map_err(|e| e.to_string())?;

    // Initialisation du module image
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;

    // Création du créateur de texture
    let texture_creator = canvas.texture_creator();

    // Chargement de l'image en tant que texture
    let texture = texture_creator.load_texture("assets/road.png")?;  // Remplace par ton image

    // Définition de la zone source (x, y, largeur, hauteur)
    let src_rect = Rect::new(0, 0, 16, 16); // Prend une partie de 200x200 à partir de (50,50)

    // Définition de la zone de destination sur l'écran
    let dest_rect = Rect::new(0, 0, 16, 16); // Affiche cette partie à (100,100)

    // Boucle principale
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

        // Effacer l'écran
        canvas.clear();

        // Dessiner une partie de l'image (src_rect → dest_rect)
        canvas.copy(&texture, src_rect, dest_rect)?;

        // Mettre à jour l'affichage
        canvas.present();

        // Petit délai pour éviter d'utiliser trop de CPU
        std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
