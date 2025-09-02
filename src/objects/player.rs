#![allow(dead_code)]
use macroquad::shapes::draw_rectangle_lines;
use vector2::Vector2;
use macroquad::prelude::{draw_rectangle};

use crate::objects::components::{Hierarchy, Transform};
use crate::shape_data::rectangle::Rectangle;

pub const PLAYER_SCALE: f64 = 48.0;
pub const DEFAULT_SPEED: f64 = 200.0;
pub const OUTLINE_SIZE: f64 = 12.0;

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

    pub fn draw(&self) {
        draw_rectangle_lines(self.shape.top_left.x as f32, self.shape.top_left.y as f32,
            self.shape.scale().x as f32, self.shape.scale().y as f32,
            OUTLINE_SIZE as f32, macroquad::color::BLACK);

        let inner_top_left: Vector2 = self.shape.top_left + Vector2::ONE * OUTLINE_SIZE / 2.0; 
        let inner_scale: Vector2 = self.shape.scale() - Vector2::ONE * OUTLINE_SIZE;

        draw_rectangle(inner_top_left.x as f32, inner_top_left.y as f32,
            inner_scale.x as f32, inner_scale.y as f32,
            macroquad::color::RED);

    }
}