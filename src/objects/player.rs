#![allow(dead_code)]
use vector2::Vector2;

use crate::objects::components::{Hierarchy, Transform};
use crate::shape_data::rectangle::Rectangle;

pub const PLAYER_SCALE: f64 = 30.0;
pub const DEFAULT_SPEED: f64 = 200.0;

pub struct Player {
    pub shape: Rectangle,
    pub hierarchy: Hierarchy,
    pub speed: f64,
    transform: Transform,
}

impl Player {
    pub fn new(position: Vector2) -> Player {
        let shape: Rectangle = 
            Rectangle::from_center(position, Vector2::ONE * PLAYER_SCALE);
        let hierarchy = Hierarchy::empty();

        Player {shape, hierarchy, speed: DEFAULT_SPEED,
            transform: Transform::at_position(position)}
    }

    pub fn move_by(&mut self, offset: Vector2) {
        self.shape.translate(offset);
    }
}