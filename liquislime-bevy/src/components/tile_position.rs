use bevy::prelude::Component;
use derive_more::{From, Into};

use crate::units::api_spec::types::TilePosition;

#[derive(Clone, Debug, Component, From, Into)]
pub struct TilePositionComponent(pub TilePosition);
