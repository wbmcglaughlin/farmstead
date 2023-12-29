use bevy::{prelude::*, window::PrimaryWindow};

use super::player::Player;

pub fn check_click_selection(
    mouse_input: Res<Input<MouseButton>>,
    mut player_entity: Query<(&GlobalTransform, &mut Transform), With<Player>>,
    query: Query<(&GlobalTransform, &Camera)>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        let (global_transform, camera) = query.single();
        if let Some(position) = q_windows.single().cursor_position() {
            let ray_pos = camera
                .viewport_to_world_2d(global_transform, position)
                .unwrap();
            for (global, mut transform) in player_entity.iter_mut() {
                transform.translation.x = ray_pos.x;
                transform.translation.y = ray_pos.y;
            }
        } else {
            println!("Cursor is not in the game window.");
        }
    }
}
