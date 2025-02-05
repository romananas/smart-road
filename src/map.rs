use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::image::LoadTexture;

pub fn load_map(canvas: &mut Canvas<Window>) -> Result<(), String> {
    let texture_creator = canvas.texture_creator();
    
    let texture = texture_creator.load_texture("assets/road.png")?;  // Remplace par ton image

    canvas.copy(&texture, None, None)?;

    Ok(())
}
