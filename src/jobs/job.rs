use bevy::prelude::*;
use bevy_ecs_tilemap::{map::TilemapSize, tiles::TilePos};

use crate::{entities::material::Material, entities::tool::Tool, map::tile::Tiles};

pub enum JobType {
    Tile(TilePos),
    EntityId(Entity),
}

pub enum JobResult {
    Tile(Tiles),
    EntityId(Vec<Material>),
}

pub struct Job {
    pub jtype: JobType,
    pub tool: Tool,
    pub result: JobResult,
}

#[derive(Component)]
pub struct Jobs {
    pub in_queue: Vec<Option<Job>>,
}

impl Jobs {
    pub fn new_from_tilemap_size(tilemap_size: &TilemapSize) -> Self {
        Jobs {
            in_queue: (0..(tilemap_size.x * tilemap_size.y))
                .map(|_| None)
                .collect(),
        }
    }
}

pub fn generate_job_queue(mut commands: Commands, mut tilemap_query: Query<&TilemapSize>) {
    let tilemap_size = tilemap_query.single();
    commands.spawn(Jobs::new_from_tilemap_size(tilemap_size));
}
