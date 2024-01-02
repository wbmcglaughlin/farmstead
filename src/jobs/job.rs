use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::TilePos;

pub enum JobType {
    Tile(TilePos),
    EntityId(Entity),
}

pub struct Job {
    jtype: JobType,
    tool: Tool,
    skill: Skill,
    materials: Option<MaterialList>,
}

#[derive(Component)]
pub struct Jobs {
    in_queue: Vec<Job>,
}

// Jobs...
// Tile based jobs.
// - tool / skill / tile pos / materials

// Entity based jobs.
// - tool / skill / entity id / -
