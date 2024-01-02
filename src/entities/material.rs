use bevy::prelude::*;

pub enum MaterialType {
    Grass,
}

pub struct Material {
    pub material_type: MaterialType,
}
