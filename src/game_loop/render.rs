use macroquad::{color::*, shapes::draw_rectangle, time::draw_fps, window::*};

use crate::{engine::{draw::*, scene::Scene}, objects::*};

pub fn render_scene(scene: &Scene, player_id: ObjectId) {
    clear_background(BLACK);
    draw_floor(32, 32);
    draw_player(scene, player_id);
    draw_anchors(scene);
    draw_enemies(scene);
    draw_fps();
}

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

fn draw_anchors(main_scene: &Scene) {
    for (anchor_id, _) in &main_scene.anchors {
        draw_anchor(main_scene, *anchor_id);
    }
}
fn draw_enemies(main_scene: &Scene) {
    for (enemy_id, _) in &main_scene.enemies {
        draw_enemy(main_scene, *enemy_id);
    }
}