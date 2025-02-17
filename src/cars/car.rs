use sdl2::{pixels::Color, rect::{Point, Rect}};
use crate::entities::*;

const BASE_VELOCITY: u32 = 4;
const SLOW_VELOCITY: u32 = BASE_VELOCITY ;
const SAFE_DISTANCE: u32 = 20;
const DETECTION_OFFSET: i32 = -0;

#[derive(Debug,Clone)]
pub enum DisplayType {
    Color(Color),
    // Texture(Texture<'a>),
}

#[derive(Debug,Clone,PartialEq, Eq)]
pub enum UpdateState {
    Moving,
    Slowing,

    Finished,
    Collided,
    Waiting,
}

#[derive(Debug,PartialEq,Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(Debug,Clone)]
pub struct Car {
    hit_box: Rect,
    detection_lower: Rect,
    detection_upper: Rect,
    sprite: DisplayType,
    state: UpdateState,
    velocity: u32,
    w_l: (u32,u32),

    path: Vec<Point>
}


impl From<Color> for DisplayType{
    fn from(color: Color) -> Self {
        DisplayType::Color(color)
    }
}

impl Car {
    pub fn new<T: Into<DisplayType>>(center: Point, w: u32, l: u32, sprite: T) -> Self {
        let hit_box = Rect::from_center(center, w, l);
        Self {
            hit_box: hit_box,
            w_l: (w,l),
            sprite: sprite.into(),
            velocity: BASE_VELOCITY,
            state: UpdateState::Moving,
            path: Vec::new(),
            detection_lower: hit_box,
            detection_upper: hit_box,
        }
    }

    pub fn set_path(&mut self,path: Vec<Point>) {
        self.path = path;
    }

    // pub fn get_state(&self) -> UpdateState {
    //     self.state.clone()
    // }

    pub fn get_detections(&self) -> Option<(Rect,Rect)>{
        Some((self.detection_lower,self.detection_upper))  
    }

    pub fn update(&mut self, others: Vec<Car>) -> UpdateState {
        if self.path.is_empty() {
            self.state = UpdateState::Finished;
            return UpdateState::Finished; // Plus de points à atteindre
        }

        let target = self.path[0]; // Prochain point à atteindre
        let position = self.hit_box.center();
    
        let direction = (
            (target.x - position.x) as f32,
            (target.y - position.y) as f32,
        );

        let card_direction = match (direction.0.abs() > direction.1.abs(),direction.0 <= 0.0,direction.1 <= 0.0) {
            (true,false,_) => Direction::East,
            (true,true,_) => Direction::West,
            (false,_,true) => Direction::South,
            (false,_,false) => Direction::North,
        };

        let distance = (direction.0.powi(2) + direction.1.powi(2)).sqrt();
    
        if distance < self.velocity as f32 {
            self.hit_box = Rect::from_center(target, self.hit_box.width(), self.hit_box.height());
            self.path.remove(0);
        } else {
            let normalized = (direction.0 / distance, direction.1 / distance);
            let movement = (
                (normalized.0 * self.velocity as f32) as i32,
                (normalized.1 * self.velocity as f32) as i32,
            );

            let new_hitbox = if card_direction == Direction::North || card_direction == Direction::South {
                Rect::from_center(
                    Point::new(position.x + movement.0, position.y + movement.1),
                    self.w_l.0,
                    self.w_l.1,
                )
            } else {
                Rect::from_center(
                    Point::new(position.x + movement.0, position.y + movement.1),
                    self.w_l.1,
                    self.w_l.0,
                )
            };
    
            // **1. Calculer l'angle de la direction**
            let angle = direction.1.atan2(direction.0); // atan2(y, x) donne l'angle en radians
    
            // **2. Calculer un vecteur perpendiculaire pour décaler à droite**
            let perpendicular = (-angle.sin(), angle.cos()); // Rotation de 90° (droite)
            let inc = if card_direction == Direction::North || card_direction == Direction::South {
                new_hitbox.height()
            } else {
                new_hitbox.width()
            };
            // **3. Placer la zone de détection devant et à droite**
            let detection_distance = inc as f32 * 1.4; // Distance devant
            let detection_x = position.x as f32 + detection_distance * angle.cos()
                + DETECTION_OFFSET as f32 * perpendicular.0;
            let detection_y = position.y as f32 + detection_distance * angle.sin()
                + DETECTION_OFFSET as f32 * perpendicular.1;
            

            let ahead_box_lower = Rect::from_center(
                Point::new(detection_x as i32, detection_y as i32),
                self.hit_box.width() + SAFE_DISTANCE,
                self.hit_box.height() + SAFE_DISTANCE,
            );
            
            let detection_distance = inc as f32 * 2.8 ; // Distance devant
            let detection_x = position.x as f32 + detection_distance * angle.cos()
                + DETECTION_OFFSET as f32 * perpendicular.0;
            let detection_y = position.y as f32 + detection_distance * angle.sin()
                + DETECTION_OFFSET as f32 * perpendicular.1;

            let ahead_box_upper = Rect::from_center(
                Point::new(detection_x as i32, detection_y as i32),
                self.hit_box.width() + SAFE_DISTANCE,
                self.hit_box.height() + SAFE_DISTANCE,
            );
            self.detection_lower = ahead_box_lower;
            self.detection_upper = ahead_box_upper;
            // **4. Vérification des collisions et de la distance de sécurité**
            for other in &others {
                if self.hit_box.has_intersection(other.hit_box) {

                    self.state = UpdateState::Collided;
                    return UpdateState::Collided;
                }
                if ahead_box_upper.has_intersection(other.detection_upper) && other.state != UpdateState::Slowing {
                    self.state = UpdateState::Slowing;
                    self.velocity = SLOW_VELOCITY;
                    self.hit_box = new_hitbox;
                    return UpdateState::Slowing;
                }
                if ahead_box_lower.has_intersection(other.get_hitbox()) || ahead_box_upper.has_intersection(other.detection_lower) || ahead_box_upper.has_intersection(other.get_hitbox()) {
                    self.state = UpdateState::Waiting;
                    return UpdateState::Waiting; // Risque Collision détectée, on ne bouge pas
                }
            }
    
            self.hit_box = new_hitbox; // Aucun obstacle, mise à jour de la position
        };
        self.velocity = BASE_VELOCITY;

        self.state = UpdateState::Moving;
        return UpdateState::Moving;
    }

}

impl Entity for Car {
    fn display(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), Box<dyn std::error::Error>> {
        if matches!(self.sprite, DisplayType::Color(_)) {

            // *! Will be used if texture is implemented
            #[allow(irrefutable_let_patterns)]
            if let DisplayType::Color(color) = self.sprite {
                canvas.set_draw_color(color);
            }
            canvas.fill_rect(self.hit_box)?;
            return Ok(());
        }
        Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Texture is not implemented yet")))
    }

    fn get_hitbox(&self) -> Rect {
        self.hit_box
    }
}