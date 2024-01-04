use bevy::prelude::*;

#[derive(Component)]
pub struct GrowingStagesAnimationIndex {
    pub start: u32,
    pub end: u32,
}
