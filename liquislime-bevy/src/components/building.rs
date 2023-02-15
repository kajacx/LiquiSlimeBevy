use bevy::prelude::*;

use crate::units::api_spec::types::SlimeAmount;

#[derive(Component, Debug)]
pub struct Building;

#[derive(Component, Debug)]
pub struct SlimeSource {
    pub amount: SlimeAmount,
    // TODO: remove this completely
}
