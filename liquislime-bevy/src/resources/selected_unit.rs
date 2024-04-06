use bevy::prelude::*;

use crate::units::UnitId;

#[derive(Clone, Debug, Default, Resource)]
pub struct SelectedUnit(pub Option<UnitId>);
