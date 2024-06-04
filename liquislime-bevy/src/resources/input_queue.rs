use bevy::prelude::*;

use crate::api::ApiPosition;

#[derive(Clone, Debug, Default, Resource)]
pub struct InputQueue(pub Vec<InputEvent>);

#[derive(Clone, Debug)]
pub enum InputEvent {
    MouseMove(ApiPosition),
    MousePressed,
    MouseJustPressed,
    MouseJustReleased,
}
