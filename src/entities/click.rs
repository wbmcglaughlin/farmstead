use bevy::prelude::*;

pub fn check_click_selection(mouse_input: Res<Input<MouseButton>>) {
    dbg!(mouse_input.pressed(MouseButton::Left));
}
