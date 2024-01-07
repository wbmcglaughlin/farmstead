use bevy::prelude::*;
use bevy_ecs_tilemap::{map::TilemapSize, tiles::TileStorage};

use crate::{
    jobs::job::{Job, Jobs},
    map::tilemap::JobLayerTileMap,
};

use self::{
    plant::{plant_bundle, GrowthStage, Plant, PlantType},
    resource::ResourceType,
};

pub mod click;
pub mod hitbox;
pub mod material;
pub mod plant;
pub mod player;
pub mod resource;
pub mod tool;

#[derive(Debug, Clone)]
pub enum TileEntityType {
    Plant(PlantType),
    Resource(ResourceType),
}

#[derive(Resource)]
pub struct EntityJobSpawnQueue {
    pub queue: Vec<Job>,
}

impl EntityJobSpawnQueue {
    pub fn new() -> Self {
        Self { queue: Vec::new() }
    }
}

#[derive(Resource)]
pub struct EntityTileStorage {
    pub storage: TileStorage,
}

impl EntityTileStorage {
    pub fn new() -> Self {
        Self {
            storage: TileStorage::empty(TilemapSize { x: 128, y: 128 }),
        }
    }
}

pub fn add_tile_entity_jobs(
    mut commands: Commands,
    mut jobs: ResMut<Jobs>,
    mut tile_entity_jobs: ResMut<EntityJobSpawnQueue>,
    mut tile_entity_mapping: ResMut<EntityTileStorage>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    tilemap_query: Query<&Transform, With<JobLayerTileMap>>,
) {
    for queue_item in tile_entity_jobs.queue.iter() {
        match &queue_item.jtype {
            crate::jobs::job::JobType::Tile(_) => todo!(),
            crate::jobs::job::JobType::TileEntity(tile_entity) => {
                let tilemap_transform = tilemap_query.single();
                match &tile_entity.etype {
                    TileEntityType::Plant(plant_type) => {
                        let texture_handle = asset_server.load(plant_type.png_file());
                        let texture_atlas = TextureAtlas::from_grid(
                            texture_handle,
                            Vec2::new(16.0, 16.0),
                            5,
                            1,
                            None,
                            None,
                        );
                        let texture_atlas_handle = texture_atlases.add(texture_atlas);
                        // Use only the subset of sprites in the sheet that make up the run animation
                        let growth_stage = GrowthStage { first: 0, last: 4 };
                        let tile_pos = tile_entity.tilepos;

                        let entity = commands
                            .spawn((
                                plant_bundle(
                                    texture_atlas_handle,
                                    growth_stage.first,
                                    tile_pos,
                                    tilemap_transform.translation,
                                ),
                                Plant {
                                    ptype: *plant_type,
                                    tile_pos,
                                    stage_progress: Timer::from_seconds(2.0, TimerMode::Repeating),
                                    planted: false,
                                },
                                growth_stage,
                            ))
                            .id();

                        tile_entity_mapping.storage.set(&tile_pos, entity);

                        jobs.in_queue.push(queue_item.clone());
                    }
                    TileEntityType::Resource(resource_type) => {
                        let texture_handle = asset_server.load(resource_type.file_path());
                        let texture_atlas = TextureAtlas::from_grid(
                            texture_handle,
                            Vec2::new(16.0, 16.0),
                            1,
                            1,
                            None,
                            None,
                        );
                        let texture_atlas_handle = texture_atlases.add(texture_atlas);
                        // Use only the subset of sprites in the sheet that make up the run animation
                        let tile_pos = tile_entity.tilepos;

                        let entity = commands
                            .spawn((
                                plant_bundle(
                                    texture_atlas_handle,
                                    0,
                                    tile_pos,
                                    tilemap_transform.translation,
                                ),
                                queue_item.clone(),
                            ))
                            .id();

                        tile_entity_mapping.storage.set(&tile_pos, entity);
                    }
                }
            }
            crate::jobs::job::JobType::Entity(_) => todo!(),
        }
    }

    tile_entity_jobs.queue.clear();
}
