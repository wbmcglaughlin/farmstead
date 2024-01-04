use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::TilePos;

use crate::{entities::material::Material, entities::tool::Tool, map::tile::Tiles};

#[derive(Debug)]
pub struct TileJob {
    pub tilepos: TilePos,
    pub tile: Tiles,
}

#[derive(Debug)]
pub struct EntityJob {
    pub entity: Entity,
    pub material: Vec<Material>,
}

#[derive(Debug)]
pub enum JobType {
    Tile(TileJob),
    Entity(EntityJob),
}

#[derive(Debug)]
pub struct Job {
    pub time: Timer,
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
