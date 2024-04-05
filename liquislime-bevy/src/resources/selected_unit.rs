use bevy::prelude::*;

use crate::units::UnitId;

#[derive(Clone, Debug, Default, Resource)]
pub struct SelectedUnit(Option<UnitId>);
