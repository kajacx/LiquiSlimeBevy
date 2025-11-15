use super::*;

pub struct GameState {
    pub grids: SlimeGrids,
}

impl GameState {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            grids: SlimeGrids::new(width, height),
        }
    }

    pub fn update(&mut self, time_passed: TimeInterval) {
        self.grids.prepare_slime_spread(time_passed);
        self.grids.spread_slime();
        self.grids.annihilate_slime();
    }
}
