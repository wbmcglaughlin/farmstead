use bevy::prelude::*;

#[derive(Resource, Debug)]
pub enum SelectionMode {
    Selection,
    Tiling,
}

pub fn switch_mode(input: Res<Input<KeyCode>>, mut mode: ResMut<SelectionMode>) {
    if input.just_pressed(KeyCode::M) {
        *mode = match *mode {
            SelectionMode::Selection => SelectionMode::Tiling,
            SelectionMode::Tiling => SelectionMode::Selection,
        };
    }
}
