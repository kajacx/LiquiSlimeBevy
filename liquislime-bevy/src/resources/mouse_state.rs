use bevy::prelude::*;

use crate::api::ApiPosition;

#[derive(Clone, Debug, Default, Resource)]
pub struct MouseState {
    pub position: Option<ApiPosition>,
    pub pressed: bool,
    pub just_pressed: bool,
    pub just_released: bool,
}
