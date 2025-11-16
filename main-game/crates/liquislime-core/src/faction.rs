use super::*;

#[derive(Debug, Clone, Copy)]
pub struct Faction {
    id: u8,
    color: Color,
}

impl Faction {
    pub fn new(id: u8, color: Color) -> Self {
        Self { id, color }
    }

    pub fn id(self) -> FactionId {
        FactionId::new(self.id)
    }

    pub fn index(self) -> usize {
        self.id as _
    }

    pub fn color(self) -> Color {
        self.color
    }
}
