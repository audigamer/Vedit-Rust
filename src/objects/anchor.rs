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

    // Transform property is temporary. I will move drawing logic to its own file later.
    // pub fn draw(&self, parent_transform: Transform) {
    //     let new_rect: Rectangle = parent_transform.transform_rect(&self.shape);

    //     draw_rectangle(new_rect.top_left.x as f32, new_rect.top_left.y as f32,
    //         new_rect.scale().x as f32, new_rect.scale().y as f32, PURPLE);
    // }
}