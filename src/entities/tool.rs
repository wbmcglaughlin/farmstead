use bevy::prelude::*;

pub enum ToolType {
    Shovel,
    Hoe,
    Pickaxe,
}

#[derive(Component)]
pub struct Tool {
    pub tool_type: ToolType,
}
