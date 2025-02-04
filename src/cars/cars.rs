use sdl2::rect::{Rect, Point};
use sdl2::render::Canvas;
use sdl2::pixels::Color;
use sdl2::video::Window;

use crate::display;

#[allow(dead_code)]

const BASE_VELOCITY: u32 = 100;
const _SLOWING_VELOCITY: u32 = BASE_VELOCITY / 2;

/// A return type to detect a collision,
/// 
/// Collision : the car is detecting another car in front of it
/// 
/// Detection : the car is detecting another car that will cross it
pub enum IntersectionType {
    Collision,
    Detection,
}

#[derive(Debug,Clone, Copy)]
pub enum Rotation {
    Right,
    Left,
}

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Car {
    pub velocity: u32,
    collision_box: Rect,
    detection_box: Rect,
    direction: Direction,
    show_col: bool,
    show_detect: bool,

    is_detecting : Option<IntersectionType>,
}

impl Car {
    pub fn new(center: Point, w: u32, l: u32) -> Self {
        Self {
            collision_box: Rect::from_center(center, w, l),
            detection_box: Rect::from_center(Point::new(center.x, center.y - (l as f32 * 1.25) as i32), w, (l as f64 * 1.5) as u32),
            velocity: BASE_VELOCITY,
            direction: Direction::North,
            show_col: false,
            show_detect: false,

            is_detecting: None,
        }
    }

    pub fn rotate(&mut self, rotation: Rotation) {
        self.direction = match (self.direction, rotation) {
            (Direction::North, Rotation::Left) => Direction::West,
            (Direction::North, Rotation::Right) => Direction::East,

            (Direction::East, Rotation::Left) => Direction::North,
            (Direction::East, Rotation::Right) => Direction::South,

            (Direction::South, Rotation::Left) => Direction::East,
            (Direction::South, Rotation::Right) => Direction::West,

            (Direction::West, Rotation::Left) => Direction::South,
            (Direction::West, Rotation::Right) => Direction::North,
        };

        // Swap width and height when rotating between horizontal & vertical
        let (w, h) = (self.collision_box.width(), self.collision_box.height());
        let center = self.collision_box.center();

        let (new_w, new_h) = match rotation {
            Rotation::Left | Rotation::Right => (h, w),
        };

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


    pub fn move_to(&mut self,x: i32,y: i32) {
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
        if self.detection_box.has_intersection(other.collision_box) {
            self.is_detecting = Some(IntersectionType::Collision);
            return Some(IntersectionType::Collision);
        }
        if self.detection_box.has_intersection(other.detection_box) {
            self.is_detecting = Some(IntersectionType::Detection);
            return Some(IntersectionType::Detection);
        }
        self.is_detecting = None;
        None
    }
}

impl display::Display for Car {
    type Error = Result<(), String>;
    fn display(&self, canvas: &mut Canvas<Window>) -> Self::Error {
        if self.show_detect {
            match &self.is_detecting {
                Some(t) => {
                    match t {
                        IntersectionType::Detection => canvas.set_draw_color(Color::YELLOW),
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
        Err(String::from("Cannot yet display sprite, not implemented yet"))
    }
}
