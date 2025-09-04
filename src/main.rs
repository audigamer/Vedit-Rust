mod objects;
mod shape_data;
mod engine;

use macroquad::prelude::*;
use vector2::Vector2;

use crate::{engine::{draw::*, scene::Scene}, objects::{components::Transform, ObjectId, Player}};

#[macroquad::main("Example")]
async fn main() {
    let mut main_scene: Scene = Scene::new();
    let player_id = main_scene.add_player(Vector2::ZERO);

    let anchor_positions = vec![
        Vector2::new(400.0, 100.0),
        Vector2::new(50.0, 0.0),
        Vector2::new(0.0, 50.0),
        Vector2::new(50.0, 50.0)
        ];

    let anchor_ids: Vec<ObjectId> = anchor_positions.iter()
        .map(|pos| main_scene.add_anchor(*pos))
        .collect();

    // Parent transform stuff only work if the scene tree gets built from parent to child in order.
    // This breaks if I swap the 2 lines of code below this.
    main_scene.append_child(anchor_ids[2], anchor_ids[3]);
    main_scene.append_child(anchor_ids[0], anchor_ids[1]);
    main_scene.append_child(anchor_ids[1], anchor_ids[2]);
    
    // TODO: Move player movement into another file
    loop {
        update_player(&mut main_scene, player_id);

        clear_background(LIGHTGRAY);
        draw_player(&mut main_scene, player_id);
        draw_anchors(&main_scene);
        draw_fps();
        next_frame().await
    }
}

fn update_player(main_scene: &mut Scene, player_id: ObjectId) {
    let player: &Player = main_scene.players.get(&player_id).unwrap();
    let player_transform: &mut Transform = main_scene.global_transforms.get_mut(&player_id).unwrap();

    if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
        player_transform.move_by(Vector2::new(player.speed, 0.0) * get_frame_time().into());
    }
    if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
        player_transform.move_by(Vector2::new(-player.speed, 0.0) * get_frame_time().into());
    }
    if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
        player_transform.move_by(Vector2::new(0.0, player.speed) * get_frame_time().into());
    }
    if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
        player_transform.move_by(Vector2::new(0.0, -player.speed) * get_frame_time().into());
    }
}

fn draw_anchors(main_scene: &Scene) {
    for (anchor_id, _) in &main_scene.anchors {
        draw_anchor(main_scene, *anchor_id);
    }
}