use bevy::prelude::*;

#[derive(Clone, Copy)]
pub enum ToolType {
    Hoe,
    Pickaxe,
}

impl ToolType {
    pub fn get_texture_index(&self) -> u32 {
        match self {
            ToolType::Hoe => 1,
            ToolType::Pickaxe => 2,
        }
    }
}

#[derive(Component)]
pub struct Tool {
    pub tool_type: ToolType,
}
