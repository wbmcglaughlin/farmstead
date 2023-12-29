use bevy::{prelude::*, window::PrimaryWindow};

use super::player::{Highlight, Player};

pub fn check_click_selection(
    mouse_input: Res<Input<MouseButton>>,
    mut player_entity: Query<&mut Transform, With<Player>>,
    mut highlight: Query<&mut Visibility, With<Highlight>>,
    query: Query<(&GlobalTransform, &Camera)>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        let (global_transform, camera) = query.single();
        if let Some(position) = q_windows.single().cursor_position() {
            let ray_pos = camera
                .viewport_to_world_2d(global_transform, position)
                .unwrap();

            for mut transform in player_entity.iter_mut() {
                let mut vis = highlight.single_mut();
                if *vis != Visibility::Visible {
                    let distance_squared = (ray_pos.x - transform.translation.x).powf(2.0)
                        + (ray_pos.y - transform.translation.y).powf(2.0);

                    if distance_squared < 9.0 {
                        *vis = Visibility::Visible;
                    }
                } else {
                    transform.translation.x = ray_pos.x;
                    transform.translation.y = ray_pos.y;
                    *vis = Visibility::Hidden;
                }
            }
        }
    }
}
