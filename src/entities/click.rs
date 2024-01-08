use crate::{
    jobs::job::{Job, JobType, Jobs, TileEntityJob, TileJob},
    map::{
        tile::Tiles,
        tilemap::{JobLayerTileMap, MainTileMap, TileComponent},
    },
    ui::{
        mode::SelectionMode,
        selection::{EntitySelectionRectangle, SelectionStatus},
    },
};
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_ecs_tilemap::prelude::*;

use super::{
    hitbox::{collision_aabb, HitBox},
    plant::PlantType,
    player::{Highlight, Player},
    tool::{Tool, ToolType},
    EntityJobSpawnQueue, EntityTileStorage, TileEntity, TileEntityType,
};

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
    mut player_entity: Query<(&mut Transform, &mut Player, &HitBox, &mut Children)>,
    mut highlight: Query<&mut Visibility, With<Highlight>>,
    mut selections: Query<&mut EntitySelectionRectangle>,
    mut jobs: ResMut<Jobs>,
) {
    for mut selection in selections.iter_mut() {
        if selection.status != SelectionStatus::Selected {
            continue;
        }
        let selection_sqaure_size = selection.get_area();
        for (transform, mut player, hitbox, children) in player_entity.iter_mut() {
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
                                if collision_aabb(transform.translation, hitbox, start) {
                                    *vis = Visibility::Visible;
                                }
                            } else {
                                player.target = Some(Vec2::new(start.x, start.y));

                                // If the target is being changed, check to see if player has a job.
                                if let Some(job) = &player.job {
                                    jobs.in_queue.push(job.clone());
                                    player.job = None;
                                }
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
        selection.reset();
    }
}

pub fn check_tiles_selection(
    mut jobs: ResMut<Jobs>,
    job_layer_tile_query: Query<
        (&TileStorage, &TilemapTileSize, &TilemapSize),
        With<JobLayerTileMap>,
    >,
    tilemap_query_tile: Query<&TileStorage, With<MainTileMap>>,
    mut tile_component_query: Query<&TileComponent>,
    mut tile_texture_query: Query<&mut TileTextureIndex>,
    mut selections: Query<&mut EntitySelectionRectangle>,
    mut entity_job_spawn_queue: ResMut<EntityJobSpawnQueue>,
    mut tile_entity_mapping: ResMut<EntityTileStorage>,
    mut tile_entity_query: Query<&TileEntity>,
) {
    for mut selection in selections.iter_mut() {
        if selection.status != SelectionStatus::Selected || selection.get_area().is_none() {
            continue;
        }
        let (tile_storage, tilemap_size, map_size) = job_layer_tile_query.single();
        let tiles_storage = tilemap_query_tile.single();
        let tile_positions = get_tile_positions(tilemap_size, map_size, &selection);
        for tile_pos in tile_positions.iter() {
            if let Some(entity) = tile_entity_mapping.storage.get(tile_pos) {
                if tile_entity_query.get_mut(entity).is_ok() {
                    continue;
                }
            }
            if let (Some(tile), Some(tiles)) =
                (tile_storage.get(tile_pos), tiles_storage.get(tile_pos))
            {
                if let (Ok(mut job_tile_texture), Ok(tile_component)) = (
                    tile_texture_query.get_mut(tile),
                    tile_component_query.get_mut(tiles),
                ) {
                    if tile_component.tile == Tiles::Field {
                        let tool_type = ToolType::Hoe;
                        let job_type = TileJob {
                            tilepos: *tile_pos,
                            tile: Tiles::Farmland,
                        };
                        jobs.in_queue.push(Job {
                            jtype: JobType::Tile(job_type),
                            tool: Some(Tool { tool_type }),
                            time: Timer::from_seconds(2.0, TimerMode::Once),
                        });
                        job_tile_texture.0 = tool_type.get_texture_index();
                    } else if tile_component.tile == Tiles::Farmland {
                        // TODO: this needs to push to the EntitySpawnJob queue.
                        // Each pass of the entity spawn queue will render the tile with an opacity,
                        // and push to the job queue.
                        let job_type = TileEntityJob {
                            tilepos: *tile_pos,
                            etype: TileEntityType::Plant(PlantType::Wheat),
                        };
                        entity_job_spawn_queue.queue.push(Job {
                            jtype: JobType::TileEntity(job_type),
                            tool: None,
                            time: Timer::from_seconds(0.0, TimerMode::Once),
                        });
                    }
                }
            }
        }

        selection.reset();
    }
}

fn get_tile_positions(
    tilemap_size: &TilemapTileSize,
    map_size: &TilemapSize,
    selection: &EntitySelectionRectangle,
) -> Vec<TilePos> {
    let halfborder = Vec2::new(
        tilemap_size.x * map_size.x as f32,
        tilemap_size.y * map_size.y as f32,
    ) / 2.0;
    let selection_start = (selection.start.unwrap() + halfborder) / tilemap_size.x;
    let selection_end = (selection.end.unwrap() + halfborder) / tilemap_size.y;

    let start_x = selection_start.x.min(selection_end.x) as usize;
    let end_x = selection_start.x.max(selection_end.x) as usize;
    let start_y = selection_start.y.min(selection_end.y) as usize;
    let end_y = selection_start.y.max(selection_end.y) as usize;

    let mut tile_positions = Vec::new();
    for x in start_x..=end_x - 1 {
        for y in start_y..=end_y - 1 {
            let tile_pos = TilePos {
                x: x as u32,
                y: y as u32,
            };
            tile_positions.push(tile_pos);
        }
    }
    tile_positions
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
