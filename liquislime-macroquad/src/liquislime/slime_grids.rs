use super::*;

#[readonly::make]
#[derive(Debug)]
pub struct SlimeGrids {
    #[readonly]
    grids: [SlimeGrid; 2],
}

#[allow(unused)]
impl SlimeGrids {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            grids: [SlimeGrid::new(width, height), SlimeGrid::new(width, height)],
        }
    }

    pub fn get_amount(&self, faction: Faction, position: TilePosition) -> SlimeAmount {
        self.grids[faction.index()].get_amount(position)
    }

    pub fn try_get_amount(&self, faction: Faction, position: TilePosition) -> Option<SlimeAmount> {
        self.grids[faction.index()].try_get_amount(position)
    }

    pub fn set_amount(&mut self, faction: Faction, position: TilePosition, amount: SlimeAmount) {
        self.grids[faction.index()].set_amount(position, amount);
    }

    pub fn try_set_amount(
        &mut self,
        faction: Faction,
        position: TilePosition,
        amount: SlimeAmount,
    ) -> Result<(), ()> {
        self.grids[faction.index()].try_set_amount(position, amount)
    }

    pub fn prepare_slime_spread(&mut self, time_passed: TimeInterval) {
        for grid in self.grids.iter_mut() {
            grid.prepare_slime_spread(time_passed)
        }
    }

    pub fn spread_slime(&mut self) {
        for grid in self.grids.iter_mut() {
            grid.spread_slime()
        }
    }

    pub fn annihilate_slime(&mut self) {
        let width = self.grids[0].width;
        let height = self.grids[0].height;

        for x in 0..width {
            for y in 0..height {
                let position = TilePosition::new(x as _, y as _);
                let amount = self.grids[0]
                    .get_amount(position)
                    .min(self.grids[1].get_amount(position));
                self.grids[0].add_amount(position, -amount);
                self.grids[1].add_amount(position, -amount);
            }
        }
    }
}
