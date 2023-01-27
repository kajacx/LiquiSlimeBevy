use bevy::prelude::*;

use super::SlimeAmount;

#[derive(Component, Debug)]
pub struct Building;

#[derive(Component, Debug)]
pub struct SlimeSource {
    pub amount: SlimeAmount,
}
