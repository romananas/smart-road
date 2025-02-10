use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::image::LoadTexture;

// ! optimisation probleme : texture is reloading from rom to ram each refresh.
// TODO : putting the texture into a var that pass in the main.
pub fn load_map(canvas: &mut Canvas<Window>) -> Result<(), String> {
    let texture_creator = canvas.texture_creator();
    
    let texture = texture_creator.load_texture("assets/road.png")?;  // Remplace par ton image

    canvas.copy(&texture, None, None)?;

    Ok(())
}