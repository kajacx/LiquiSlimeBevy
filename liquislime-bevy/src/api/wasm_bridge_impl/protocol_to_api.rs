use super::bindgen;
use crate::api;

impl From<bindgen::Faction> for api::Faction {
    fn from(value: bindgen::Faction) -> Self {
        Self::new(value.id)
    }
}

impl From<bindgen::SlimeAmount> for api::SlimeAmount {
    fn from(value: bindgen::SlimeAmount) -> Self {
        Self(value.amount)
    }
}

impl From<bindgen::TilePosition> for api::TilePosition {
    fn from(value: bindgen::TilePosition) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

impl From<bindgen::Position> for api::Position {
    fn from(value: bindgen::Position) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

impl From<bindgen::TimeInterval> for api::TimeInterval {
    fn from(value: bindgen::TimeInterval) -> Self {
        Self(value.fragments)
    }
}
