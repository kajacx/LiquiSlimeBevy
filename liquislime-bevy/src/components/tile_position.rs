use bevy::prelude::Component;
use derive_more::{From, Into};

use crate::api::ApiTilePosition;

#[derive(Clone, Debug, Component, From, Into)]
pub struct TilePositionComponent(pub ApiTilePosition);
