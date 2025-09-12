use macroquad::time::get_frame_time;
use vector2::Vector2;

use crate::{engine::*, objects::{components::Transform, ObjectId, Player}};

pub fn update_player(main_scene: &mut Scene, player_id: ObjectId) {
    let player: &Player = main_scene.players.get(&player_id).unwrap();
    let player_transform: &mut Transform = main_scene.global_transforms.get_mut(&player_id).unwrap();

    let action_map = ActionMap::default();

    if action_map.is_action_active("right") {
        player_transform.move_by(Vector2::new(player.speed, 0.0) * get_frame_time().into());
    }
    if action_map.is_action_active("left") {
        player_transform.move_by(Vector2::new(-player.speed, 0.0) * get_frame_time().into());
    }
    if action_map.is_action_active("down") {
        player_transform.move_by(Vector2::new(0.0, player.speed) * get_frame_time().into());
    }
    if action_map.is_action_active("up") {
        player_transform.move_by(Vector2::new(0.0, -player.speed) * get_frame_time().into());
    }
}

pub fn update_transform(main_scene: &mut Scene, id: ObjectId, offset: Vector2) {
    let transform: &Transform = main_scene.local_transforms.get(&id).unwrap();
    let new_transform = Transform::at_position(transform.position + offset);
    main_scene.local_transforms.insert(id, new_transform);
}