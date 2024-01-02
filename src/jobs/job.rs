use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::TilePos;

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
}

#[derive(Component)]
pub struct Jobs {
    pub in_queue: Vec<Job>,
}

impl Jobs {
    pub fn new() -> Self {
        Jobs {
            in_queue: Vec::new(),
        }
    }
}

pub fn generate_job_queue(mut commands: Commands) {
    commands.spawn(Jobs::new());
}
