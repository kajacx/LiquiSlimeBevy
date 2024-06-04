use bevy::prelude::*;

use crate::components::UnitId;

#[derive(Clone, Debug, Default, Resource)]
pub struct SelectedUnit(pub Option<UnitId>);
