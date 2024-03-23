use super::*;
use derive_more::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Faction {
    id: u8,
}

impl Faction {
    pub fn get_own_faction() -> Self {
        Self::from_protocol(crate::protocol::get_own_faction())
    }

    pub fn as_protocol(self) -> crate::protocol::Faction {
        crate::protocol::Faction { id: self.id }
    }

    pub fn from_protocol(faction: crate::protocol::Faction) -> Self {
        Self { id: faction.id }
    }
}
