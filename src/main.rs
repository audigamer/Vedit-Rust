mod objects;
mod shape_data;
mod engine;
mod game_loop;

use macroquad::prelude::*;
use vector2::Vector2;

use crate::{engine::*, game_loop::*, objects::ObjectId};

#[macroquad::main("Example")]
async fn main() {
    let mut main_scene: Scene = Scene::new();
    let player_id = main_scene.add_player(Vector2::ZERO);

    let enemy_positions = vec![
        Vector2::new(480.0, 96.0),
        Vector2::new(48.0, 0.0),
        Vector2::new(0.0, 48.0),
        Vector2::new(24.0, 24.0)
        ];

    let enemy_ids: Vec<ObjectId> = enemy_positions.iter()
        .map(|pos| main_scene.add_enemy(*pos))
        .collect();

    main_scene.append_child(enemy_ids[0], enemy_ids[1]);
    main_scene.append_child(enemy_ids[0], enemy_ids[2]);
    main_scene.append_child(enemy_ids[1], enemy_ids[3]);

    main_scene.initialize_transforms();
    
    loop {
        update_player(&mut main_scene, player_id);

        // TODO: Make updating transform work in any order.
        // Right now, the child must be updated before the parent, or else the transforms will break.
        // update_transform(&mut main_scene, enemy_ids[1],
        //     Vector2::UP * get_frame_time().into() * 25.0);
        // update_transform(&mut main_scene, enemy_ids[0],
        //     Vector2::RIGHT * get_frame_time().into() * 50.0);

        render_scene(&main_scene, player_id);
        next_frame().await
    }
}