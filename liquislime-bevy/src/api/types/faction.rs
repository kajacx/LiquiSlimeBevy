use super::*;
use derive_more::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Faction {
    id: u8,
}

impl Faction {
    pub fn new(id: u8) -> Self {
        Self { id }
    }

    pub fn index(self) -> usize {
        self.id as _
    }
}
