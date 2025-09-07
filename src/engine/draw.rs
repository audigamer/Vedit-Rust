use macroquad::{color::*, shapes::*};
use vector2::Vector2;

use crate::{engine::scene::Scene, objects::{components::Transform, enemy::Enemy, ObjectId, Player}};
use crate::shape_data::{rectangle::Rectangle, circle::Circle};


// Shapes
pub fn draw_rect_shape(rect: &Rectangle, color: Color) {
    draw_rectangle(rect.top_left.x as f32, rect.top_left.y as f32,
        rect.scale().x as f32, rect.scale().y as f32, color);
}

pub fn draw_rect_outline(rect: &Rectangle, outline_width: f64, color: Color) {
    draw_rectangle_lines(rect.top_left.x as f32, rect.top_left.y as f32,
        rect.scale().x as f32, rect.scale().y as f32, outline_width as f32, color);
}

pub fn draw_outlined_rect(rect: &Rectangle, outline_width: f64, inner_color: Color, outer_color: Color) {
    draw_rect_outline(&rect, outline_width, outer_color);
    
    let inner_rect: Rectangle = Rectangle {
        top_left: rect.top_left + Vector2::ONE * outline_width / 2.0,
        bottom_right: rect.bottom_right - Vector2::ONE * outline_width / 2.0,
    };
    draw_rect_shape(&inner_rect, inner_color);
}

pub fn draw_circle_shape(circle: &Circle, color: Color) {
    draw_circle(circle.center.x as f32, circle.center.y as f32, circle.radius as f32, color);
}

pub fn draw_circle_outline(circle: &Circle, outline_width: f64, color: Color) {
    draw_circle_lines(circle.center.x as f32, circle.center.y as f32, circle.radius as f32, outline_width as f32, color);
}

pub fn draw_outlined_circle(circle: &Circle, outline_width: f64, inner_color: Color, outer_color: Color) {
    let inner_circle: Circle = Circle::new(circle.center, circle.radius - outline_width);
    let slighly_bigger_inner_circle: Circle = Circle::new(circle.center, inner_circle.radius + 0.5);
    draw_circle_outline(&inner_circle, outline_width, outer_color);
    draw_circle_shape(&slighly_bigger_inner_circle, inner_color);
}


// Objects

pub const PLAYER_COLOR: Color = Color::from_hex(0xff3333);
pub const ENEMY_COLOR: Color = Color::from_hex(0x3333ff);

pub fn draw_floor(tiles_x: i32, tiles_y: i32) {
    let color1: Color = Color::from_hex(0xf7f7f7);
    draw_rectangle(0.0, 0.0, tiles_x as f32* 48.0, tiles_y as f32 * 48.0, color1);
    let color2: Color = Color::from_hex(0xddddff);
    for i in 0..tiles_x {
        for j in 0..tiles_y {
            if (i + j) % 2 == 0 {
                draw_rectangle(j as f32 * 48.0, i as f32 * 48.0,
                48.0, 48.0, color2)
            }
        }
    }
}

pub fn draw_player(main_scene: &Scene, player_id: ObjectId) {
    let player: &Player = main_scene.players.get(&player_id).unwrap();
    let transform: &Transform = main_scene.global_transforms.get(&player_id).unwrap();
    draw_outlined_rect(&transform.transform_rect(&player.shape),
        Player::OUTLINE_SIZE, PLAYER_COLOR, BLACK);
}

// pub fn draw_all<T: GameObject>(main_scene: &Scene, map: &HashMap<ObjectId, T>) {
//     for (id, _) in map {
//         draw_anchor(main_scene, *id);
//     }
// }
pub fn draw_anchor(main_scene: &Scene, anchor_id: ObjectId) {
    let anchor = main_scene.anchors.get(&anchor_id).unwrap();
    let transform = main_scene.global_transforms.get(&anchor_id).unwrap();
    draw_rect_shape(&transform.transform_rect(&anchor.shape), PURPLE);
}


pub fn draw_enemy(main_scene: &Scene, enemy_id: ObjectId) {
    let enemy = main_scene.enemies.get(&enemy_id).unwrap();
    let transform = main_scene.global_transforms.get(&enemy_id).unwrap();
    draw_outlined_circle(&transform.transform_circle(&enemy.shape), 
        Enemy::OUTLINE_SIZE, ENEMY_COLOR, BLACK);
}

