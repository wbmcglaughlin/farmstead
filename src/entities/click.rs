use bevy::{prelude::*, window::PrimaryWindow};

use crate::ui::selection::SelectionRectangle;

use super::player::{Highlight, Player};

pub fn check_click_selection(
    mouse_input: Res<Input<MouseButton>>,
    mut player_entity: Query<(&mut Transform, &mut Player, &mut Children)>,
    mut highlight: Query<&mut Visibility, With<Highlight>>,
    mut selections: Query<&mut SelectionRectangle>,
    query: Query<(&GlobalTransform, &Camera)>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    if let Some(position) = q_windows.single().cursor_position() {
        let (global_transform, camera) = query.single();

        let ray_pos = camera
            .viewport_to_world_2d(global_transform, position)
            .unwrap();

        for mut selection in selections.iter_mut() {
            if mouse_input.just_pressed(MouseButton::Left) {
                selection.set_start(ray_pos);
                selection.end = None;
            }

            if mouse_input.pressed(MouseButton::Left) {
                selection.set_end(ray_pos);
            }

            if mouse_input.just_released(MouseButton::Left) {
                if let Some(start) = selection.start {
                    if start.distance_squared(ray_pos) > 50.0 {
                        selection.set_end(ray_pos);
                        // TODO: Do logic for selection here.
                        return;
                    }
                }

                for (transform, mut player, children) in player_entity.iter_mut() {
                    // Iterate over the children, there should only be one currently.
                    for child in &children {
                        // Get the query element, this will throw an error if it doesnt contain a
                        // highlight, but there is only one.
                        if let Ok(mut vis) = highlight.get_mut(*child) {
                            if *vis != Visibility::Visible {
                                // TODO: need to handle this better. Hard coded currently.
                                let distance_squared = (ray_pos.x - transform.translation.x)
                                    .powf(2.0)
                                    + (ray_pos.y - transform.translation.y).powf(2.0);

                                if distance_squared < 9.0 {
                                    *vis = Visibility::Visible;
                                }
                            } else {
                                player.target = Some(Vec2::new(ray_pos.x, ray_pos.y));
                                *vis = Visibility::Hidden;
                            }
                        }
                    }
                }
            }
        }
    }
}
