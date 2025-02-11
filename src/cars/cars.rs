use sdl2::rect::{Rect, Point};
use sdl2::render::Canvas;
use sdl2::pixels::Color;
use sdl2::video::Window;
use sdl2::render::Texture;
use sdl2::image::LoadTexture;


use crate::display;

#[allow(dead_code)]

const BASE_VELOCITY: u32 = 3;
const _SLOWING_VELOCITY: u32 = BASE_VELOCITY / 2;

/// A return type to detect a collision,
/// 
/// Collision : the car is detecting another car in front of it
/// 
/// Detection : the car is detecting another car that will cross it

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IntersectionType {
    Collision,
    SawDetectionBox,
    SawCollisionBox,

}

#[derive(Clone, Copy, Debug,PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}
pub struct Car<'a> {
    pub velocity: u32,
    collision_box: Rect,
    detection_box: Rect,
    direction: Direction,
    show_col: bool,
    show_detect: bool,
    is_detecting: Option<IntersectionType>,
    texture: Option<Texture<'a>>,
}

impl<'a> Car<'a> {

    pub fn new(center: Point, texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>) -> Self {
        let texture = texture_creator.load_texture("assets/car.png").ok(); // Image directement chargée ici
        let texture_query = texture.as_ref().map(|t| t.query());
        let texture_width = texture_query.map(|q| q.width).unwrap_or(0);
        let texture_height = texture_query.map(|q| q.height).unwrap_or(0);

        Self {
            collision_box: Rect::from_center(center, texture_width, texture_height),
            detection_box: Rect::from_center(Point::new(center.x, center.y - (texture_height as f32 * 1.25) as i32), texture_width, (texture_height as f64 * 1.5) as u32),
            velocity: BASE_VELOCITY,
            direction: Direction::North,
            show_col: false,
            show_detect: false,
            is_detecting: None,
            texture,
        }
    }

    pub fn set_direction(&mut self,direction: Direction) {
        let center = self.collision_box.center();
        let (mut new_h,mut new_w) = (self.collision_box.height(),self.collision_box.width());
        use Direction::*;
        if self.direction == direction {
            return;
        }
        match (self.direction,direction) {
            (North,South)|(South,North)|(West,East)|(East,West) => {},
            _ => {
                let tmp = new_h;
                new_h = new_w;
                new_w = tmp;
            },
        }
        self.direction = direction;
        self.collision_box = Rect::from_center(center, new_w, new_h);
        self.detection_box = match self.direction {
             Direction::North => Rect::from_center(Point::new(center.x, center.y - (new_h as f32 * 1.25) as i32), new_w, (new_h as f64 * 1.5) as u32),
             Direction::South => Rect::from_center(Point::new(center.x, center.y + (new_h as f32 * 1.25) as i32), new_w, (new_h as f64 * 1.5) as u32),
             Direction::East => Rect::from_center(Point::new(center.x + (new_w as f32 * 1.25) as i32, center.y), (new_w as f64 * 1.5) as u32, new_h),
             Direction::West => Rect::from_center(Point::new(center.x - (new_w as f32 * 1.25) as i32, center.y), (new_w as f64 * 1.5) as u32, new_h),
         };
    }

    pub fn show_collisions(&mut self, b: bool) {
        self.show_col = b;
    }

    pub fn show_detections(&mut self, b: bool) {
        self.show_detect = b;
    }

    pub fn go_to(&mut self,target: Point) -> bool {
        let mut center = self.collision_box.center();
        let direction = match get_direction(self.collision_box.center(), target) {
            Some(direction) => direction,
            None => return true,
        };
        self.set_direction(direction);

        let pts_dist = (((target.x - center.x).pow(2) + (target.y - center.y).pow(2)) as f64).sqrt();
        let distance = self.velocity as f64;

        if pts_dist <= distance {
            center.x = target.x;
            center.y = target.y;
            return true;
        }
         // Calcul du vecteur direction (flottants pour la précision)
         let dx = (target.x - center.x) as f64;
         let dy = (target.y - center.y) as f64;
 
         // Calcul de la norme du vecteur
         let magnitude = (dx.powi(2) + dy.powi(2)).sqrt();
 
         // Normalisation du vecteur direction
         let unit_x = dx / magnitude;
         let unit_y = dy / magnitude;

        center.x = (center.x as f64 + unit_x * distance).round() as i32;
        center.y = (center.y as f64 + unit_y * distance).round() as i32;

        self.move_to(center.x, center.y);
        false
    }

    pub fn move_to(&mut self, x: i32, y: i32) {
        self.collision_box = Rect::from_center(Point::new(x, y), self.collision_box.width(), self.collision_box.height());
        self.detection_box = match self.direction {
            Direction::North => Rect::from_center(Point::new(x, y - (self.collision_box.height() as f32 * 1.25) as i32), self.collision_box.width(), (self.collision_box.height() as f64 * 1.5) as u32),
            Direction::South => Rect::from_center(Point::new(x, y + (self.collision_box.height() as f32 * 1.25) as i32), self.collision_box.width(), (self.collision_box.height() as f64 * 1.5) as u32),
            Direction::East => Rect::from_center(Point::new(x + (self.collision_box.width() as f32 * 1.25) as i32, y), (self.collision_box.width() as f64 * 1.5) as u32, self.collision_box.height()),
            Direction::West => Rect::from_center(Point::new(x - (self.collision_box.width() as f32 * 1.25) as i32, y), (self.collision_box.width() as f64 * 1.5) as u32, self.collision_box.height()),
        };
    }
    
    /// Detect if the car will intersect with another car
    /// and return Some(IntersectionType) if it sees a car
    /// or None if it sees nothing
    pub fn has_intersection(&mut self, other: &Self) -> Option<IntersectionType> {

        if self.collision_box.has_intersection(other.collision_box) {
            self.is_detecting = Some(IntersectionType::Collision);
            return Some(IntersectionType::Collision);
        }
        if self.detection_box.has_intersection(other.detection_box) || self.detection_box.has_intersection(other.collision_box) {
            self.is_detecting = Some(IntersectionType::SawDetectionBox);
            return Some(IntersectionType::SawDetectionBox);
        }
        self.is_detecting = None;
        None
    }
    pub fn state_check(&mut self, other: &Self) {
        let previous_state = self.is_detecting; 
        let new_state = self.has_intersection(other);
    
        if previous_state != new_state {
            match new_state {
                Some(IntersectionType::Collision) => println!("Collision detected"),
                Some(IntersectionType::SawDetectionBox) => println!("A car is comming"),
                Some(IntersectionType::SawCollisionBox) => println!("A car is in front"),
                None => println!("No car detected"),
            }
        }
    }
    
}

fn get_direction(p1: Point, p2: Point) -> Option<Direction> {
    let dx = (p2.x - p1.x).abs(); // Distance horizontale
    let dy = (p2.y - p1.y).abs(); // Distance verticale

    if dx == 0 && dy == 0 {
        return None;
    }

    if dy > dx {
        if p2.y < p1.y {
            Some(Direction::North)
        } else {
            Some(Direction::South)
        }
    } else {
        if p2.x > p1.x {
            Some(Direction::East)
        } else {
            Some(Direction::West)
        }
    }
}


impl<'a> display::Display for Car<'a> {
    type Error = Result<(), String>;

    fn display(&self, canvas: &mut Canvas<Window>) -> Self::Error {
        // Afficher la box de collision et la détection
        if self.show_detect {
            match &self.is_detecting {
                Some(t) => {
                    match t {
                        IntersectionType::SawDetectionBox => canvas.set_draw_color(Color::YELLOW),
                        IntersectionType::SawCollisionBox => canvas.set_draw_color(Color::RGB(255, 92, 0)),
                        IntersectionType::Collision => canvas.set_draw_color(Color::RED),
                    }
                }
                None => canvas.set_draw_color(Color::GRAY),
            }
            match canvas.draw_rect(self.detection_box) {
                Ok(_) => {},
                Err(e) => return Err(e),
            }
        }
        if self.show_col {
            canvas.set_draw_color(Color::GREEN);
            match canvas.draw_rect(self.collision_box) {
                Ok(_) => {}
                Err(e) => return Err(e),
            }
        }

        // Affichage de la texture centrée sur la boxe de collision
        if let Some(texture) = &self.texture {
            // Récupérer les dimensions de la texture
            let texture_query = texture.query();
            let texture_width = texture_query.width;
            let texture_height = texture_query.height;

            // Calculer la position de la texture pour qu'elle soit centrée sur la boxe de collision
            let x = self.collision_box.center().x - (texture_width / 2) as i32;
            let y = self.collision_box.center().y - (texture_height / 2) as i32;

            let rotation_angle = 90.0;

            // Afficher la texture centrée sur la boîte de collision
            let dest_rect = Rect::new(x, y, texture_width, texture_height);
            canvas.copy_ex(texture, None, dest_rect, rotation_angle, None, false, false)?;        
        }

        Ok(())
    }
}



pub const RED: Color = Color::RGB(255, 0, 0);
pub const BLUE: Color = Color::RGB(0, 0, 255);
pub const GREEN: Color = Color::RGB(0, 255, 0);
pub const YELLOW: Color = Color::RGB(255, 255, 0);

pub fn road_point(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, x: i32, y: i32, color: Color) {
    canvas.set_draw_color(color);
    canvas.fill_rect(Rect::new(x, y, 5, 5)).unwrap();
}
