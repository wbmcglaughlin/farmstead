use bevy::ecs::component::Component;

#[derive(Component, Clone, Copy)]
pub enum Tiles {
    Field,
    Grass,
    Farmland,
    Dirt,
    Stone,
    Rock,
}
