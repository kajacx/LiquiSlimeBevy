use super::*;
use derive_more::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Faction {
    pub id: u8,
}

impl Faction {
    pub fn get_own_faction() -> Self {
        crate::get_own_faction()
    }
}
