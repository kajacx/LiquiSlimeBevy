use super::bindgen;
use crate::api;

impl From<api::Faction> for bindgen::Faction {
    fn from(value: api::Faction) -> Self {
        Self {
            id: value.index() as _,
        }
    }
}

impl From<api::SlimeAmount> for bindgen::SlimeAmount {
    fn from(value: api::SlimeAmount) -> Self {
        Self { amount: value.0 }
    }
}

impl From<api::TilePosition> for bindgen::TilePosition {
    fn from(value: api::TilePosition) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

impl From<api::Position> for bindgen::Position {
    fn from(value: api::Position) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

impl From<api::TimeInterval> for bindgen::TimeInterval {
    fn from(value: api::TimeInterval) -> Self {
        Self { fragments: value.0 }
    }
}

impl From<api::Settings> for bindgen::SettingValues {
    fn from(value: api::Settings) -> Self {
        vec![(
            "amount".into(),
            bindgen::SettingValue::SlimeAmount(value.amount.into()),
        )]
    }
}
