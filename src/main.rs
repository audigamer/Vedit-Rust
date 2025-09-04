mod objects;
mod shape_data;
mod engine;

use macroquad::prelude::*;
use vector2::Vector2;

use crate::{engine::scene::Scene, objects::{ObjectId, Player}};

#[macroquad::main("Example")]
async fn main() {
    //let mut player_added_from_data: Player = Player::new(Vector2::new(24.0, 24.0));
    let mut main_scene: Scene = Scene::new();
    let player_id = main_scene.add_player(Vector2::ZERO);

    // TODO: Move player movement into another file

    loop {
        update_player(&mut main_scene, player_id);

        clear_background(LIGHTGRAY);
        draw_player(&mut main_scene, player_id);
        draw_fps();
        next_frame().await
    }
}

fn update_player(main_scene: &mut Scene, player_id: ObjectId) {
    let player: &mut Player = main_scene.players.get_mut(&player_id).unwrap();

    if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            player.move_by(Vector2::new(player.speed, 0.0) * get_frame_time().into());
        }
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            player.move_by(Vector2::new(-player.speed, 0.0) * get_frame_time().into());
        }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            player.move_by(Vector2::new(0.0, player.speed) * get_frame_time().into());
        }
        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            player.move_by(Vector2::new(0.0, -player.speed) * get_frame_time().into());
        }
}

fn draw_player(main_scene: &Scene, player_id: ObjectId) {
    let player: &Player = main_scene.players.get(&player_id).unwrap();
    player.draw();
}