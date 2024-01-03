use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum SelectionMode {
    #[default]
    Selection,
    Tiling,
}

pub fn switch_mode(
    input: Res<Input<KeyCode>>,
    state: Res<State<SelectionMode>>,
    mut next_state: ResMut<NextState<SelectionMode>>,
) {
    if input.just_pressed(KeyCode::M) {
        match *state.get() {
            SelectionMode::Selection => next_state.set(SelectionMode::Tiling),
            SelectionMode::Tiling => next_state.set(SelectionMode::Selection),
        };
    }
}
