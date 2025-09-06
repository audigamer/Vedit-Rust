use vector2::Vector2;
use crate::{shape_data::rectangle::Rectangle};

/*
Struct that only contains a square shape. Used for grouping objects, such as enemies
that follow the same movement pattern.
*/
pub struct Anchor {
    pub shape: Rectangle,
}

pub const ANCHOR_SIZE: f64 = 24.0;

impl Anchor {
    pub fn new() -> Self {
        Self { shape: Rectangle::from_center(Vector2::ZERO, Vector2::ONE * ANCHOR_SIZE) }
    }
}