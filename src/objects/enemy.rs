use vector2::Vector2;

use crate::shape_data::circle::Circle;

pub struct Enemy {
    pub shape: Circle,
}


impl Enemy {
    pub const ENEMY_RADIUS: f64 = 13.0;
    pub const OUTLINE_SIZE: f64 = 6.0;
    pub fn new() -> Self {
        Enemy { shape: Circle::new(Vector2::ZERO, Self::ENEMY_RADIUS) }
    }
}