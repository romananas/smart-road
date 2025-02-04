use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::image::LoadTexture;

pub fn load_map(canvas: &mut Canvas<Window>,) -> Result<(), String> {
    // Création du créateur de texture
    let texture_creator = canvas.texture_creator();
    
    // Charger l'image (modifie le chemin si nécessaire)
    let texture = texture_creator.load_texture("assets/road.png")?;  // Remplace par ton image

    // Définition de la zone source (x, y, largeur, hauteur)
    let src_rect = Rect::new(0, 0, 10, 10); // Partie de l'image à afficher

    // Définition de la zone de destination (où afficher cette portion)
    let dest_rect = Rect::new(0, 0, 10, 10); // Position à (100, 100)

    // Copier la texture sur le canvas
    canvas.copy(&texture, src_rect, dest_rect)?;

    Ok(())
}
