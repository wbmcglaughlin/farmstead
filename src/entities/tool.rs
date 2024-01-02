use bevy::prelude::*;

pub enum ToolType {
    Shovel,
    Hoe,
    Pickaxe,
}

#[Component]
pub struct Tool;
