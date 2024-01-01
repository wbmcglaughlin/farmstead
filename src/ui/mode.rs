use bevy::prelude::*;

#[derive(Resource)]
pub enum SelectionMode {
    Selection,
    Tiling,
}
