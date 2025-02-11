extern crate sdl2;

mod map;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use smart_road::cars::cars::BLUE;
use smart_road::cars::cars::GREEN;
use smart_road::cars::cars::RED;
use smart_road::cars::cars::YELLOW;
use smart_road::display::Display;
use std::time::Duration;

use smart_road::cars::*;

use smart_road::cars::cars::Direction;
use smart_road::cars::cars::road_point;



fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Affichage d'une partie de l'image", 1100, 1100)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().present_vsync().build().map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;
    // Charger le créateur de texture avant de créer la voiture
    let texture_creator = canvas.texture_creator();
    let mut car = Car::new(Point::new(500, 500), &texture_creator); // Make the car

    car.show_collisions(true);
    car.show_detections(true);
    
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {keycode : Some(Keycode::W),..} => car.set_direction(Direction::North),
                Event::KeyDown {keycode : Some(Keycode::S),..} => car.set_direction(Direction::South),
                Event::KeyDown {keycode : Some(Keycode::D),..} => car.set_direction(Direction::East),
                Event::KeyDown {keycode : Some(Keycode::A),..} => car.set_direction(Direction::West),
                _ => {}
            }
        }

        canvas.clear();

        map::load_map(&mut canvas)?;

        //R1
        //R1-1
        road_point(&mut canvas, 370, 20, RED); // p1
        road_point(&mut canvas, 370, 355, RED); // p2
        road_point(&mut canvas, 20, 355, RED); // p3
        //R1-2
        road_point(&mut canvas, 450, 20, RED);
        road_point(&mut canvas, 430, 540, RED);
        road_point(&mut canvas, 425, 1000, RED);
        //R1-3
        road_point(&mut canvas, 520, 20, RED);
        road_point(&mut canvas, 520, 590, RED);
        road_point(&mut canvas, 1080, 590, RED);


        //R2
        //R2-1
        road_point(&mut canvas, 1080, 360, BLUE);
        road_point(&mut canvas, 740, 360, BLUE);
        road_point(&mut canvas, 740, 20, BLUE);
        //R2-2
        road_point(&mut canvas, 1080, 440, BLUE);
        road_point(&mut canvas, 430, 430, BLUE);
        road_point(&mut canvas, 20, 420, BLUE);
        //R2-3
        road_point(&mut canvas, 1080, 510, BLUE);
        road_point(&mut canvas, 510, 510, BLUE);
        road_point(&mut canvas, 505, 1000, BLUE);

        //R3
        //R3-1
        road_point(&mut canvas, 720, 1000, GREEN);
        road_point(&mut canvas, 720, 740, GREEN);
        road_point(&mut canvas, 1080, 730, GREEN);
        //R3-2
        road_point(&mut canvas, 644, 1000, GREEN);
        road_point(&mut canvas, 655, 500, GREEN);
        road_point(&mut canvas, 670, 20, GREEN);
        //R3-3
        road_point(&mut canvas, 570, 1000, GREEN);
        road_point(&mut canvas, 570, 500, GREEN);
        road_point(&mut canvas, 20, 500, GREEN);

        //R4
        //R4-1
        road_point(&mut canvas, 20, 730,YELLOW);
        road_point(&mut canvas, 350, 730,YELLOW);
        road_point(&mut canvas, 350, 1000,YELLOW);
        //R4-2
        road_point(&mut canvas, 20, 650,YELLOW);
        road_point(&mut canvas, 550, 650,YELLOW);
        road_point(&mut canvas, 1080, 650,YELLOW);
        //R4-3
        road_point(&mut canvas, 20, 575,YELLOW);
        road_point(&mut canvas, 590, 575,YELLOW);
        road_point(&mut canvas, 590, 20,YELLOW);










        // Move the car towards a target
        car.go_to(Point::new(1100, 550));

        // Display the car
        car.display(&mut canvas)?;

        canvas.present();

        // Control frame rate (60 FPS)
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}





// STRUCT TO USE FOR MAKE CAR ROAD 

//let mut road_map: HashMap<&str, RoadSection> = HashMap::new();
//
//road_map.insert("R1", RoadSection {
//    routes: vec![
//        Route { points: vec![Point { x: 370, y: 20 }, Point { x: 370, y: 355 }, Point { x: 20, y: 355 }] },
//        Route { points: vec![Point { x: 450, y: 20 }, Point { x: 430, y: 540 }, Point { x: 425, y: 1000 }] },
//        Route { points: vec![Point { x: 520, y: 20 }, Point { x: 520, y: 590 }, Point { x: 1080, y: 590 }] },
//    ]
//});
//
//road_map.insert("R2", RoadSection {
//    routes: vec![
//        Route { points: vec![Point { x: 1080, y: 360 }, Point { x: 740, y: 360 }, Point { x: 740, y: 20 }] },
//        Route { points: vec![Point { x: 1080, y: 440 }, Point { x: 430, y: 430 }, Point { x: 20, y: 420 }] },
//        Route { points: vec![Point { x: 1080, y: 510 }, Point { x: 510, y: 510 }, Point { x: 505, y: 1000 }] },
//    ]
//});
//
//road_map.insert("R3", RoadSection {
//    routes: vec![
//        Route { points: vec![Point { x: 720, y: 1000 }, Point { x: 720, y: 740 }, Point { x: 1080, y: 730 }] },
//        Route { points: vec![Point { x: 644, y: 1000 }, Point { x: 655, y: 500 }, Point { x: 670, y: 20 }] },
//        Route { points: vec![Point { x: 570, y: 1000 }, Point { x: 570, y: 500 }, Point { x: 20, y: 500 }] },
//    ]
//});
//
//road_map.insert("R4", RoadSection {
//    routes: vec![
//        Route { points: vec![Point { x: 20, y: 730 }, Point { x: 350, y: 730 }, Point { x: 350, y: 1000 }] },
//        Route { points: vec![Point { x: 20, y: 650 }, Point { x: 550, y: 650 }, Point { x: 1080, y: 650 }] },
//        Route { points: vec![Point { x: 20, y: 575 }, Point { x: 590, y: 575 }, Point { x: 590, y: 20 }] },
//    ]
//});