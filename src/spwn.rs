#![allow(dead_code)]

use sdl2::rect::Point;
use crate::cars;

// TODO: Fill all point formated as .0 = spawn point, .1, .2 = road points 
const N_S: [(i32,i32); 3] = [(450,20),(430,540),(425,1000)];
const N_E: [(i32,i32); 3] = [(520,20),(520,590),(1080,590)];
const N_W: [(i32,i32); 3] = [(370,20),(370,355),(20,355)];

const S_N: [(i32,i32); 3] = [(644,1000),(655,500),(670,20)];
const S_E: [(i32,i32); 3] = [(720,1000),(720,740),(1080,730)];
const S_W: [(i32,i32); 3] = [(570,1000),(570,500),(20,500)];

const E_W: [(i32,i32); 3] = [(1080,440),(430,430),(20,420)];
const E_N: [(i32,i32); 3] = [(1080,360),(740,360),(740,20)];
const E_S: [(i32,i32); 3] = [(1080,510),(510,510),(505,1000)];

const W_E: [(i32,i32); 3] = [(20,650),(550,650),(1080,650)];
const W_N: [(i32,i32); 3] = [(20,575),(590,575),(590,20)];
const W_S: [(i32,i32); 3] = [(20,730),(350,730),(350,1000)];

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