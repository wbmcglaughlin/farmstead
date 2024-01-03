use bevy::prelude::*;

pub enum MaterialType {
    Wheat,
}

pub struct Material {
    pub material_type: MaterialType,
}
