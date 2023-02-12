use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct MoveOnClick {
    pub mouse_button: MouseButton,
}
