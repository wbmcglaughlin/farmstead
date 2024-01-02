use bevy::{prelude::*, window::PrimaryWindow};

use crate::ui::{
    mode::SelectionMode,
    selection::{EntitySelectionRectangle, SelectionStatus},
};

use super::player::{Highlight, Player};

pub fn click_drag_handler(
    mouse_input: Res<Input<MouseButton>>,
    mode: Res<State<SelectionMode>>,
    mut selections: Query<&mut EntitySelectionRectangle>,
    query: Query<(&GlobalTransform, &Camera)>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    if let Some(position) = q_windows.single().cursor_position() {
        let (global_transform, camera) = query.single();

        let ray_pos = camera
            .viewport_to_world_2d(global_transform, position)
            .unwrap();

        let bottom_right_pos = match *mode.get() {
            SelectionMode::Selection => ray_pos,
            SelectionMode::Tiling => {
                let tile_size = 16.0; // Assuming tile size is 16x16
                let x = (ray_pos.x / tile_size).ceil() * tile_size;
                let y = (ray_pos.y / tile_size).floor() * tile_size;
                Vec2::new(x, y)
            }
        };

        let top_left_pos = match *mode.get() {
            SelectionMode::Selection => ray_pos,
            SelectionMode::Tiling => {
                let tile_size = 16.0; // Assuming tile size is 16x16
                let x = (ray_pos.x / tile_size).floor() * tile_size;
                let y = (ray_pos.y / tile_size).ceil() * tile_size;
                Vec2::new(x, y)
            }
        };

        for mut selection in selections.iter_mut() {
            if mouse_input.just_pressed(MouseButton::Left) {
                selection.set_start(top_left_pos);
                selection.end = None;
                selection.status = SelectionStatus::Clicked;
            } else if mouse_input.pressed(MouseButton::Left) {
                selection.set_end(bottom_right_pos);
                selection.status = SelectionStatus::Selecting;
            } else if mouse_input.just_released(MouseButton::Left) {
                selection.status = SelectionStatus::Selected;
            }
        }
    }
}

pub fn check_entities_selection(
    mode: Res<State<SelectionMode>>,
    mut player_entity: Query<(&mut Transform, &mut Player, &mut Children)>,
    mut highlight: Query<&mut Visibility, With<Highlight>>,
    mut selections: Query<&mut EntitySelectionRectangle>,
) {
    for mut selection in selections.iter_mut() {
        if selection.status != SelectionStatus::Selected && *mode == SelectionMode::Selection {
            continue;
        }
        let selection_sqaure_size = selection.get_area();
        for (transform, mut player, children) in player_entity.iter_mut() {
            // Iterate over the children, there should only be one currently.
            for child in &children {
                // Get the query element, this will throw an error if it doesnt contain a
                // highlight, but there is only one.
                if let Ok(mut vis) = highlight.get_mut(*child) {
                    // Check the players selection visibility, if the selection exists,
                    // set the player target.
                    if selection_sqaure_size.is_none() || selection_sqaure_size.unwrap() < 10.0 {
                        if let Some(start) = selection.start {
                            if *vis != Visibility::Visible {
                                // TODO: need to handle this better. Hard coded currently.
                                let distance_squared = (start.x - transform.translation.x)
                                    .powf(2.0)
                                    + (start.y - transform.translation.y).powf(2.0);

                                if distance_squared < 9.0 {
                                    *vis = Visibility::Visible;
                                }
                            } else {
                                player.target = Some(Vec2::new(start.x, start.y));
                                *vis = Visibility::Hidden;
                            }
                        }
                    } else {
                        let selection_start = selection.start.unwrap();
                        let selection_end = selection.end.unwrap();
                        let player_position =
                            Vec2::new(transform.translation.x, transform.translation.y);

                        if check_intersection(player_position, selection_start, selection_end) {
                            *vis = Visibility::Visible;
                        }
                    }
                }
            }
        }
        selection.status = SelectionStatus::Ready;
        selection.start = None;
        selection.end = None;
    }
}

pub fn check_tiles_selection() {
    todo!();
}

pub fn check_intersection(
    player_position: Vec2,
    selection_start: Vec2,
    selection_end: Vec2,
) -> bool {
    player_position.x >= selection_start.x.min(selection_end.x)
        && player_position.x <= selection_start.x.max(selection_end.x)
        && player_position.y >= selection_start.y.min(selection_end.y)
        && player_position.y <= selection_start.y.max(selection_end.y)
}
