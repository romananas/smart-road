#![allow(dead_code)]

use sdl2::rect::Point;
use crate::cars;

// TODO: Fill all point formated as .0 = spawn point, .1, .2 = road points 
const N_S: [(i32,i32); 3] = [(0,0),(0,0),(0,0)];
const N_E: [(i32,i32); 3] = [(0,0),(0,0),(0,0)];
const N_W: [(i32,i32); 3] = [(0,0),(0,0),(0,0)];

const S_N: [(i32,i32); 3] = [(0,0),(0,0),(0,0)];
const S_E: [(i32,i32); 3] = [(0,0),(0,0),(0,0)];
const S_W: [(i32,i32); 3] = [(0,0),(0,0),(0,0)];

const E_W: [(i32,i32); 3] = [(0,0),(0,0),(0,0)];
const E_N: [(i32,i32); 3] = [(0,0),(0,0),(0,0)];
const E_S: [(i32,i32); 3] = [(0,0),(0,0),(0,0)];

const W_E: [(i32,i32); 3] = [(0,0),(0,0),(0,0)];
const W_N: [(i32,i32); 3] = [(0,0),(0,0),(0,0)];
const W_S: [(i32,i32); 3] = [(0,0),(0,0),(0,0)];

fn get_points(from: cars::Direction,to: cars::Direction) -> Result<(Point,Vec<Point>),String> {
    use cars::Direction::*;
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

pub fn spwn(from: cars::Direction,to: cars::Direction,car_w: u32,car_l: u32) -> Result<(cars::Car, Vec<Point>), Box<dyn std::error::Error>> {
    let (c,path) = get_points(from, to)?;
	// Example implementation
	let car = cars::Car::new(c,car_w,car_l);
	Ok((car, path))
}