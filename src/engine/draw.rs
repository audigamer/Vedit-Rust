use macroquad::prelude::*;
use vector2::Vector2;

use crate::{engine::scene::Scene, objects::{components::Transform, ObjectId, Player}, shape_data::rectangle::Rectangle};

pub fn draw_rect_shape(rect: &Rectangle, color: Color) {
    draw_rectangle(rect.top_left.x as f32, rect.top_left.y as f32,
        rect.scale().x as f32, rect.scale().y as f32, color);
}

pub fn draw_rect_outline(rect: &Rectangle, outline_width: f32, color: Color) {
    draw_rectangle_lines(rect.top_left.x as f32, rect.top_left.y as f32,
        rect.scale().x as f32, rect.scale().y as f32, outline_width, color);
}

pub fn draw_outlined_rect(rect: &Rectangle, outline_width: f64, inner_color: Color, outer_color: Color) {
    draw_rect_outline(&rect, outline_width as f32, outer_color);
    
    let inner_rect: Rectangle = Rectangle {
        top_left: rect.top_left + Vector2::ONE * outline_width / 2.0,
        bottom_right: rect.bottom_right - Vector2::ONE * outline_width / 2.0,
    };
    draw_rect_shape(&inner_rect, inner_color);
}

pub fn draw_player(main_scene: &Scene, player_id: ObjectId) {
    let player: &Player = main_scene.players.get(&player_id).unwrap();
    let transform: &Transform = main_scene.global_transforms.get(&player_id).unwrap();
    draw_outlined_rect(&transform.transform_rect(&player.shape), Player::OUTLINE_SIZE, RED, BLACK);
}

pub fn draw_anchor(main_scene: &Scene, anchor_id: ObjectId) {
    let anchor = main_scene.anchors.get(&anchor_id).unwrap();
    let transform = main_scene.global_transforms.get(&anchor_id).unwrap();
    draw_rect_shape(&transform.transform_rect(&anchor.shape), PURPLE);
}

