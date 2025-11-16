#[derive(Debug, Clone, Copy)]
pub struct FactionId {
    pub id: u8,
}

impl FactionId {
    pub fn new(id: u8) -> Self {
        Self { id }
    }

    pub fn index(self) -> usize {
        self.id as _
    }
}
