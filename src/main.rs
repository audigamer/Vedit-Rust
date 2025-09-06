mod objects;
mod shape_data;
mod engine;

use macroquad::prelude::*;
use vector2::Vector2;

use crate::{engine::{draw::*, scene::Scene, update::{update_player, update_transform}}, objects::ObjectId};

#[macroquad::main("Example")]
async fn main() {
    let mut main_scene: Scene = Scene::new();
    let player_id = main_scene.add_player(Vector2::ZERO);

    let enemy_positions = vec![
        Vector2::new(400.0, 100.0),
        Vector2::new(50.0, 0.0),
        Vector2::new(0.0, 50.0),
        Vector2::new(50.0, 50.0)
        ];

    let enemy_ids: Vec<ObjectId> = enemy_positions.iter()
        .map(|pos| main_scene.add_enemy(*pos))
        .collect();

    main_scene.append_child(enemy_ids[0], enemy_ids[1]);
    main_scene.append_child(enemy_ids[1], enemy_ids[2]);
    main_scene.append_child(enemy_ids[2], enemy_ids[3]);
    
    set_default_filter_mode(FilterMode::Nearest);
    loop {
        update_player(&mut main_scene, player_id);

        // TODO: Make updating transform work in any order.
        // Right now, the child must be updated before the parent, or else the transforms will break.
        update_transform(&mut main_scene, enemy_ids[1],
            Vector2::UP * get_frame_time().into() * 25.0);
        update_transform(&mut main_scene, enemy_ids[0],
            Vector2::RIGHT * get_frame_time().into() * 50.0);


        clear_background(LIGHTGRAY);
        draw_player(&main_scene, player_id);
        draw_anchors(&main_scene);
        draw_enemies(&main_scene);
        draw_fps();
        next_frame().await
    }
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