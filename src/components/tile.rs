use bevy::prelude::*;

use super::SlimeAmount;

#[derive(Component, Debug)]
pub struct Tile {
    slime_amount: SlimeAmount,
    incoming_slime: Cell<SlimeAmount>,
}
