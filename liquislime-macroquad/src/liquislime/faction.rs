#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Faction(pub u8);

impl Faction {
    pub fn new(id: u8) -> Self {
        Self(id)
    }

    pub fn index(self) -> usize {
        self.0 as _
    }
}
