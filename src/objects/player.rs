use vector2::Vector2;
use crate::shape_data::rectangle::Rectangle;

pub struct Player {
    pub shape: Rectangle,
    pub speed: f64,
}

impl Player {
    pub const PLAYER_SCALE: f64 = 42.0;
    pub const DEFAULT_SPEED: f64 = 200.0;
    pub const OUTLINE_SIZE: f64 = 12.0;


    pub fn new() -> Player {
        let shape: Rectangle = 
            Rectangle::from_center(Vector2::ZERO, Vector2::ONE * Self::PLAYER_SCALE);

        Player {shape, speed: Self::DEFAULT_SPEED }
    }
}