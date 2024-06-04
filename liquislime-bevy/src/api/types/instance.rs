use derive_more::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ApiInstance(pub u32);

impl ApiInstance {
    pub fn new(id: u32) -> Self {
        Self(id)
    }
}
