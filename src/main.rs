mod objects;
mod shape_data;
mod engine;

use macroquad::prelude::*;
use vector2::Vector2;

use crate::{engine::{draw::*, scene::Scene, update::update_player}, objects::ObjectId};

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

    main_scene.append_child(anchor_ids[0], anchor_ids[1]);
    main_scene.append_child(anchor_ids[1], anchor_ids[2]);
    main_scene.append_child(anchor_ids[2], anchor_ids[3]);
    
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



fn draw_anchors(main_scene: &Scene) {
    for (anchor_id, _) in &main_scene.anchors {
        draw_anchor(main_scene, *anchor_id);
    }
}