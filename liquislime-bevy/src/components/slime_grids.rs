use bevy::prelude::*;

use crate::api::{SlimeAmount, TilePosition};

use super::SlimeGrid;

#[derive(Component, Debug)]
pub struct SlimeGrids {
    grids: [SlimeGrid; 2],
}

#[allow(unused)]
impl SlimeGrids {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            grids: [SlimeGrid::new(width, height), SlimeGrid::new(width, height)],
        }
    }

    pub fn get_amount(&self, grid: usize, position: TilePosition) -> SlimeAmount {
        self.grids[grid].get_amount(position)
    }

    pub fn try_get_amount(&self, grid: usize, position: TilePosition) -> Option<SlimeAmount> {
        self.grids[grid].try_get_amount(position)
    }

    pub fn set_amount(&mut self, grid: usize, position: TilePosition, amount: SlimeAmount) {
        self.grids[grid].set_amount(position, amount);
    }

    pub fn try_set_amount(
        &mut self,
        grid: usize,
        position: TilePosition,
        amount: SlimeAmount,
    ) -> Result<(), ()> {
        self.grids[grid].try_set_amount(position, amount)
    }

    pub fn add_amount(&mut self, grid: usize, position: TilePosition, amount: SlimeAmount) {
        self.grids[grid].add_amount(position, amount);
    }

    pub fn try_add_amount(
        &mut self,
        grid: usize,
        position: TilePosition,
        amount: SlimeAmount,
    ) -> Result<(), ()> {
        self.grids[grid].try_add_amount(position, amount)
    }

    pub fn prepare_slime_spread(&mut self) {
        for grid in self.grids.iter_mut() {
            grid.prepare_slime_spread()
        }
    }

    pub fn spread_slime(&mut self) {
        for grid in self.grids.iter_mut() {
            grid.spread_slime()
        }
    }
}
