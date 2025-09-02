mod objects;
mod shape_data;

use macroquad::prelude::*;
use vector2::Vector2;

use crate::objects::Player;

#[macroquad::main("Example")]
async fn main() {

    // TODO: Move player movement into another file
    let mut player: Player = Player::new(Vector2::new(24.0, 24.0));
    loop {
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

        clear_background(LIGHTGRAY);

        player.draw();
        draw_fps();
        next_frame().await
    }
}