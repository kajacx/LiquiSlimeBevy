use bevy::prelude::*;

use crate::api::{ApiTilePosition, SlimeAmount};

#[derive(Debug)]
pub struct SlimeGrid {
    width: usize,
    height: usize,

    slime_amounts: Vec<SlimeAmount>,   // row major (y * height + x)
    slime_additions: Vec<SlimeAmount>, // used for spreading the slime
}

#[allow(unused)]
impl SlimeGrid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,

            slime_amounts: vec![SlimeAmount::new(); width * height],
            slime_additions: vec![SlimeAmount::new(); width * height],
        }
    }

    pub fn get_amount(&self, position: ApiTilePosition) -> SlimeAmount {
        self.slime_amounts[self.get_index(position)]
    }

    pub fn try_get_amount(&self, position: ApiTilePosition) -> Option<SlimeAmount> {
        if self.in_range(position) {
            Some(self.get_amount(position))
        } else {
            None
        }
    }

    pub fn set_amount(&mut self, position: ApiTilePosition, amount: SlimeAmount) {
        let index = self.get_index(position);
        self.slime_amounts[index] = amount.non_negative();
    }

    pub fn try_set_amount(
        &mut self,
        position: ApiTilePosition,
        amount: SlimeAmount,
    ) -> Result<(), ()> {
        if self.in_range(position) {
            self.set_amount(position, amount);
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn add_amount(&mut self, position: ApiTilePosition, amount: SlimeAmount) {
        let index = self.get_index(position);
        let amount = self.slime_amounts[index] + amount;
        self.slime_amounts[index] = amount.non_negative();
    }

    pub fn try_add_amount(
        &mut self,
        position: ApiTilePosition,
        amount: SlimeAmount,
    ) -> Result<(), ()> {
        if self.in_range(position) {
            self.add_amount(position, amount);
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn in_range(&self, position: ApiTilePosition) -> bool {
        (position.x as usize) < self.width && (position.y as usize) < self.height
    }

    fn get_index(&self, position: ApiTilePosition) -> usize {
        position.x as usize + position.y as usize * self.width
    }

    pub fn prepare_slime_spread(&mut self) {
        for y in 0..(self.height - 1) {
            for x in 0..self.width {
                self.prepare_spread_between(
                    self.get_index(ApiTilePosition::new(x as _, y as _)),
                    self.get_index(ApiTilePosition::new(x as _, (y + 1) as _)),
                );
            }
        }

        for y in 0..self.height {
            for x in 0..(self.width - 1) {
                self.prepare_spread_between(
                    self.get_index(ApiTilePosition::new((x + 1) as _, y as _)),
                    self.get_index(ApiTilePosition::new(x as _, y as _)),
                );
            }
        }
    }

    fn prepare_spread_between(&mut self, i1: usize, i2: usize) {
        let current_amount = self.slime_amounts[i1];
        let neighbor_amount = self.slime_amounts[i2];

        // from current to neighbor
        let moved_amount = (current_amount - neighbor_amount) / 12;

        self.slime_additions[i1] -= moved_amount;
        self.slime_additions[i2] += moved_amount;
    }

    pub fn spread_slime(&mut self) {
        for index in 0..self.slime_additions.len() {
            self.slime_amounts[index] += self.slime_additions[index];
            self.slime_additions[index] = SlimeAmount::from_integer(0);
        }
    }
}
