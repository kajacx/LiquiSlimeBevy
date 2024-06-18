use bevy::prelude::*;

use crate::{
    api::{ApiFaction, ApiPosition, ApiTilePosition, SlimeAmount},
    WORLD_HEIGHT, WORLD_WIDTH,
};

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

    pub fn get_amount(&self, faction: ApiFaction, position: ApiTilePosition) -> SlimeAmount {
        self.grids[faction.index()].get_amount(position)
    }

    pub fn try_get_amount(
        &self,
        faction: ApiFaction,
        position: ApiTilePosition,
    ) -> Option<SlimeAmount> {
        self.grids[faction.index()].try_get_amount(position)
    }

    pub fn set_amount(
        &mut self,
        faction: ApiFaction,
        position: ApiTilePosition,
        amount: SlimeAmount,
    ) {
        self.grids[faction.index()].set_amount(position, amount);
    }

    pub fn try_set_amount(
        &mut self,
        faction: ApiFaction,
        position: ApiTilePosition,
        amount: SlimeAmount,
    ) -> Result<(), ()> {
        self.grids[faction.index()].try_set_amount(position, amount)
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

    pub fn annihilate_slime(&mut self) {
        for x in 0..WORLD_WIDTH {
            for y in 0..WORLD_HEIGHT {
                let position = ApiTilePosition::new(x as _, y as _);
                let amount = self.grids[0]
                    .get_amount(position)
                    .min(self.grids[1].get_amount(position));
                self.grids[0].add_amount(position, -amount);
                self.grids[1].add_amount(position, -amount);
            }
        }
    }
}
