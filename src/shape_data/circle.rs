#![allow(dead_code)]

use vector2::Vector2;
use core::fmt;

pub struct Circle {
    pub center: Vector2,
    pub radius: f64,
}

impl Circle {
    pub fn new(center: Vector2, radius: f64) -> Circle {
        Circle {center, radius}
    }
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Circle(center: {}, radius: {})", self.center, self.radius)
    }
}
