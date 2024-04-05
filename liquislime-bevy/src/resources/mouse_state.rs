use bevy::prelude::*;

use crate::api::Position;

#[derive(Clone, Debug, Default, Resource)]
pub struct MouseState {
    pub position: Option<Position>,
}
