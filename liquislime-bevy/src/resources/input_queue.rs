use bevy::prelude::*;

use crate::api::Position;

#[derive(Clone, Debug, Default, Resource)]
pub struct InputQueue(pub Vec<InputEvent>);

#[derive(Clone, Debug)]
pub enum InputEvent {
    MouseMove(Position),
    MousePressed,
    MouseJustPressed,
    MouseJustReleased,
}
