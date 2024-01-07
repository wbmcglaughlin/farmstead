use crate::{
    entities::{plant::Plant, player::Player, EntityTileStorage, TileEntityType},
    map::tilemap::{JobLayerTileMap, MainTileMap, TileComponent},
};

use self::job::Job;
use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::{TileStorage, TileTextureIndex};

pub mod job;

#[derive(Resource)]
pub struct JobCleanUpQueue {
    pub queue: Vec<Job>,
}

impl JobCleanUpQueue {
    pub fn new() -> Self {
        Self { queue: Vec::new() }
    }
}

pub fn clean_jobs(
    jobtile_map_query: Query<&TileStorage, With<JobLayerTileMap>>,
    tilemap_query: Query<&TileStorage, With<MainTileMap>>,
    mut tile_query: Query<(&mut TileTextureIndex, &mut TileComponent)>,
    mut job_tile_query: Query<&mut TileTextureIndex, Without<TileComponent>>,
    mut job_cleanup_queue: ResMut<JobCleanUpQueue>,
) {
    let jobtile_storage = jobtile_map_query.single();
    let tile_storage = tilemap_query.single();

    for job in job_cleanup_queue.queue.iter() {
        match &job.jtype {
            job::JobType::Tile(tile_job) => {
                if let (Some(job_tile), Some(tile)) = (
                    jobtile_storage.get(&tile_job.tilepos),
                    tile_storage.get(&tile_job.tilepos),
                ) {
                    // TODO: if either of these two Ok's fall through unexpected behaviour will occur.
                    if let Ok(mut job_tile_texture) = job_tile_query.get_mut(job_tile) {
                        job_tile_texture.0 = 0;
                    }
                    if let Ok((mut tile_texture, mut tiles)) = tile_query.get_mut(tile) {
                        tile_texture.0 = tile_job.tile.get_texture_index();
                        tiles.update_tile_type(tile_job.tile);
                    }
                }
            }
            job::JobType::TileEntity(_) => todo!(),
            job::JobType::Entity(_) => todo!(),
        }
    }

    job_cleanup_queue.queue.clear();
}
