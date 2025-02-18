#![allow(dead_code)]

use sdl2::{image::LoadTexture, pixels::Color, rect::{Point, Rect}, render::Canvas, video::Window};
use crate::cars::Car;

const BORDER_UP_LEFT: i32 = -40;
const BORDER_DOWN_RIGHT: i32 = 1100;

// W : LEFT
// E : RIGHT

// Done
const N_S: [(i32,i32); 3] = [(440,BORDER_UP_LEFT),(440,440),(440,BORDER_DOWN_RIGHT)];
const N_E: [(i32,i32); 3] = [(520,BORDER_UP_LEFT),(520,590),(BORDER_DOWN_RIGHT,590)];
const N_W: [(i32,i32); 3] = [(360,BORDER_UP_LEFT),(360,360),(BORDER_UP_LEFT,360)];

// Done
const S_N: [(i32,i32); 3] = [(660,BORDER_DOWN_RIGHT),(660,660),(660,BORDER_UP_LEFT)];
const S_E: [(i32,i32); 3] = [(730,BORDER_DOWN_RIGHT),(730,730),(BORDER_DOWN_RIGHT,730)];
const S_W: [(i32,i32); 3] = [(580,BORDER_DOWN_RIGHT),(580,510),(BORDER_UP_LEFT,510)];

// Done
const E_W: [(i32,i32); 3] = [(BORDER_DOWN_RIGHT,430),(430,430),(BORDER_UP_LEFT,430)];
const E_N: [(i32,i32); 3] = [(BORDER_DOWN_RIGHT,360),(750,360),(750,BORDER_UP_LEFT)];
const E_S: [(i32,i32); 3] = [(BORDER_DOWN_RIGHT,510),(510,510),(510,BORDER_DOWN_RIGHT)];

// Not Done
const W_E: [(i32,i32); 3] = [(BORDER_UP_LEFT,660),(660,660),(BORDER_DOWN_RIGHT,660)];
const W_N: [(i32,i32); 3] = [(BORDER_UP_LEFT,580),(590,580),(590,BORDER_UP_LEFT)];
const W_S: [(i32,i32); 3] = [(BORDER_UP_LEFT,740),(350,740),(350,BORDER_DOWN_RIGHT)];

#[derive(Debug,PartialEq,Clone, Copy)]
pub enum Direction {
    North,
    South,
    West,
    East,
}

#[allow(dead_code)]
impl Direction {
    /// Generate a random direction
    pub fn random() -> Self {
        match rand::random_range(0..=3) {
            0 => Self::North,
            1 => Self::South,
            2 => Self::East,
            _ => Self::West
        }
    }

    /// Generate a random direction but no the choosen one.
    pub fn random_without(other: Self) -> Self {
        let generated =  Self::random();
        if generated == other {
            return Self::random_without(other);
        }
        generated
    }
}

pub fn show_points(canvas: &mut Canvas<Window>) -> Result<(),String> {
    let values = vec![N_S,N_E,N_W,S_N,S_E,S_W,E_W,E_N,E_S,W_E,W_N,W_S];
    for va in values {
        for (i,vb) in va.iter().enumerate() {
            match i {
                0 => canvas.set_draw_color(Color::RED),
                2 => canvas.set_draw_color(Color::CYAN),
                _ => canvas.set_draw_color(Color::GREY),
            }
            let p = Point::new(vb.0, vb.1);
            canvas.draw_rect(Rect::from_center(p, 3, 3))?;
        }
    };
    Ok(())
}

fn get_points(from: Direction,to: Direction) -> Result<(Point,Vec<Point>),String> {
    use Direction::*;
    let points = match (from, to) {
        (North, South) => N_S,
        (North, East) => N_E,
        (North, West) => N_W,

        (South, North) => S_N,
        (South, East) => S_E,
        (South, West) => S_W,

        (East, West) => E_W,
        (East, North) => E_N,
        (East, South) => E_S,

        (West, East) => W_E,
        (West, North) => W_N,
        (West, South) => W_S,

        _ => return Err(String::from("invalid direction combination")),
    };
    Ok((Point::new(points[0].0, points[0].1),vec![Point::new(points[1].0, points[1].1),Point::new(points[2].0, points[2].1)]))
}


pub fn spawn_car<'a>(from: Direction, to: Direction, car_w: u32, car_l: u32) -> Result<Car<'a>, String> {
    let (strt, path) = get_points(from, to)?;
    let mut car = Car::new(strt, car_w, car_l, Color::BLUE);

    car.set_path(path);
    Ok(car)
}

pub fn load_map(canvas : &mut Canvas<Window>) -> Result<(), String> {
    let texture_creator = canvas.texture_creator();

    let texture = texture_creator.load_texture("assets/road.png")?;


    canvas.copy(&texture, None, None)?;

    Ok(())
}