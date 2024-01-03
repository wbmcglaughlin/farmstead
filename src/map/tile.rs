use bevy::ecs::component::Component;

#[derive(Debug, Component, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tiles {
    Field,
    Grass,
    Farmland,
    Dirt,
    Stone,
    Rock,
}

impl Tiles {
    pub fn get_texture_index(&self) -> u32 {
        match self {
            Tiles::Field => 0,
            Tiles::Grass => 1,
            Tiles::Farmland => 2,
            Tiles::Dirt => 3,
            Tiles::Stone => 4,
            Tiles::Rock => 5,
        }
    }
}
