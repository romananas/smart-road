#![allow(dead_code)]

use sdl2::{pixels::Color, rect::{Point, Rect}, render::Canvas, video::Window};
use crate::cars::Car;

const BORDER_UP_LEFT: i32 = 20;
const BORDER_DOWN_RIGHT: i32 = 1080;

// W : LEFT
// E : RIGHT

// Done
const N_S: [(i32,i32); 3] = [(430,BORDER_UP_LEFT),(430,430),(430,BORDER_DOWN_RIGHT)];
const N_E: [(i32,i32); 3] = [(510,BORDER_UP_LEFT),(510,590),(BORDER_DOWN_RIGHT,590)];
const N_W: [(i32,i32); 3] = [(350,BORDER_UP_LEFT),(350,350),(BORDER_UP_LEFT,350)];

// Done
const S_N: [(i32,i32); 3] = [(670,BORDER_DOWN_RIGHT),(670,670),(670,BORDER_UP_LEFT)];
const S_E: [(i32,i32); 3] = [(750,BORDER_DOWN_RIGHT),(750,750),(BORDER_DOWN_RIGHT,750)];
const S_W: [(i32,i32); 3] = [(590,BORDER_DOWN_RIGHT),(590,510),(BORDER_UP_LEFT,510)];

// Done
const E_W: [(i32,i32); 3] = [(BORDER_DOWN_RIGHT,430),(430,430),(BORDER_UP_LEFT,430)];
const E_N: [(i32,i32); 3] = [(BORDER_DOWN_RIGHT,350),(750,350),(750,BORDER_UP_LEFT)];
const E_S: [(i32,i32); 3] = [(BORDER_DOWN_RIGHT,510),(510,510),(510,BORDER_DOWN_RIGHT)];

// Not Done
const W_E: [(i32,i32); 3] = [(BORDER_UP_LEFT,670),(670,670),(BORDER_DOWN_RIGHT,670)];
const W_N: [(i32,i32); 3] = [(BORDER_UP_LEFT,590),(590,590),(590,BORDER_UP_LEFT)];
const W_S: [(i32,i32); 3] = [(BORDER_UP_LEFT,750),(350,750),(350,BORDER_DOWN_RIGHT)];

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

pub fn spawn_car(from: Direction, to: Direction, car_w: u32,car_l: u32) -> Result<Car,String> {
    let (strt,path) = get_points(from, to)?;
    let mut car = Car::new(strt, car_w, car_l, sdl2::pixels::Color::BLUE);
    car.set_path(path);
    Ok(car)
}