use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::TilePos;

use crate::{
    entities::material::Material,
    entities::{tool::Tool, TileEntityType},
    map::tile::Tiles,
};

#[derive(Debug, Clone)]
pub struct TileJob {
    pub tilepos: TilePos,
    pub tile: Tiles,
}

#[derive(Debug, Clone)]
pub struct EntityJob {
    pub entity: Entity,
    pub material: Vec<Material>,
}

#[derive(Debug, Clone)]
pub struct TileEntityJob {
    pub tilepos: TilePos,
    pub etype: TileEntityType,
}

#[derive(Debug, Clone)]
pub enum JobType {
    Tile(TileJob),
    TileEntity(TileEntityJob),
    Entity(EntityJob),
}

#[derive(Debug, Clone)]
pub struct Job {
    pub time: Timer,
    pub jtype: JobType,
    pub tool: Option<Tool>,
}

#[derive(Resource)]
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
