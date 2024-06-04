#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ApiFaction(pub u8);

impl ApiFaction {
    pub fn new(id: u8) -> Self {
        Self(id)
    }

    pub fn index(self) -> usize {
        self.0 as _
    }
}
