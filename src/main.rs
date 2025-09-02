mod objects;
mod shape_data;

use macroquad::prelude::*;
use vector2::Vector2;

use crate::objects::Player;

#[macroquad::main("Example")]
async fn main() {
    let mut player: Player = Player::new(Vector2::new(20.0, 20.0));
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
        draw_rectangle(player.shape.position().x as f32, player.shape.position().y as f32,
            player.shape.scale().x as f32, player.shape.scale().y as f32, RED);
        // draw_circle(200., 200., 50., BLUE);
        // draw_poly(400., 300., 6, 40., 0., GREEN);
        //draw_text("Hello", 20., 20., 30., BLACK);
        draw_fps();
        next_frame().await
    }
}