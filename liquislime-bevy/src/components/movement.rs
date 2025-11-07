use bevy::prelude::Component;

use crate::api::ApiPosition;

#[derive(Clone, Debug, Component)]
pub struct MovementComponent {
    pub movement_speed: f32,
    pub moving_to: Option<ApiPosition>,
}
