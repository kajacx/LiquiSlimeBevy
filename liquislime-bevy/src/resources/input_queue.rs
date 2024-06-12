use crate::api::ApiPosition;
use bevy::prelude::*;

#[derive(Clone, Debug, Default, Resource)]
pub struct InputQueue(pub Vec<InputEvent>);

// TODO: Remove allow after non-mouse events are added
#[allow(clippy::enum_variant_names)]
#[derive(Clone, Debug)]
pub enum InputEvent {
    MouseMove(ApiPosition),
    MousePressed,
    MouseJustPressed,
    MouseJustReleased,
}
