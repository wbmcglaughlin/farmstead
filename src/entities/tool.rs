use bevy::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum ToolType {
    Hoe,
    Pickaxe,
    Shovel,
    Bag,
}

impl ToolType {
    pub fn get_texture_index(&self) -> u32 {
        match self {
            ToolType::Hoe => 1,
            ToolType::Pickaxe => 2,
            ToolType::Shovel => 3,
            ToolType::Bag => 4,
        }
    }
}

#[derive(Debug, Component, Clone)]
pub struct Tool {
    pub tool_type: ToolType,
}
