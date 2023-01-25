use bevy::prelude::*;

use super::SlimeAmount;

#[derive(Component, Debug)]
pub struct SlimeGrid {
    width: usize,
    height: usize,

    slime_amounts: Vec<SlimeAmount>,   // row major (y * height + x)
    slime_additions: Vec<SlimeAmount>, // used for spreading the slime
}

impl SlimeGrid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,

            slime_amounts: vec![SlimeAmount(0); width * height],
            slime_additions: vec![SlimeAmount(0); width * height],
        }
    }

    #[inline]
    pub fn get_amount(&self, x: usize, y: usize) -> SlimeAmount {
        self.slime_amounts[self.get_index(x, y)]
    }

    pub fn try_get_amount(self, x: usize, y: usize) -> Option<SlimeAmount> {
        if self.in_range(x, y) {
            Some(self.get_amount(x, y))
        } else {
            None
        }
    }

    #[inline]
    pub fn set_amount(&mut self, x: usize, y: usize, amount: SlimeAmount) {
        let index = self.get_index(x, y);
        self.slime_amounts[index] = amount;
    }

    pub fn try_set_amount(&mut self, x: usize, y: usize, amount: SlimeAmount) -> Result<(), ()> {
        if self.in_range(x, y) {
            self.set_amount(x, y, amount);
            Ok(())
        } else {
            Err(())
        }
    }

    #[inline]
    pub fn in_range(&self, x: usize, y: usize) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }

    #[inline]
    fn get_index(&self, x: usize, y: usize) -> usize {
        x + y * self.width
    }

    pub fn prepare_slime_spread(&mut self) {
        for y in 0..(self.height - 1) {
            for x in 0..self.width {
                self.prepare_spread_between(self.get_index(x, y), self.get_index(x, y + 1));
            }
        }

        for y in 0..self.height {
            for x in 0..(self.width - 1) {
                self.prepare_spread_between(self.get_index(x + 1, y), self.get_index(x, y));
            }
        }
    }

    #[inline]
    fn prepare_spread_between(&mut self, i1: usize, i2: usize) {
        let current_amount = self.slime_amounts[i1];
        let neighbor_amount = self.slime_amounts[i2];

        // from current to neighbor
        let moved_amount = (current_amount - neighbor_amount) / 12;

        self.slime_amounts[i1] += moved_amount;
        self.slime_amounts[i2] -= moved_amount;
    }

    pub fn spread_slime(&mut self) {
        for index in 0..self.slime_additions.len() {
            self.slime_amounts[index] += self.slime_additions[index];
            self.slime_additions[index] = SlimeAmount(0);
        }
    }
}
