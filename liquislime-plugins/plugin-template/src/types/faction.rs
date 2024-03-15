use super::*;
use derive_more::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Faction {
    id: u8,
}

impl Faction {
    pub const fn faction0() -> Self {
        Self { id: 0 }
    }

    pub const fn faction1() -> Self {
        Self { id: 1 }
    }

    pub fn as_protocol(self) -> crate::protocol::Faction {
        crate::protocol::Faction { id: self.id }
    }
}
