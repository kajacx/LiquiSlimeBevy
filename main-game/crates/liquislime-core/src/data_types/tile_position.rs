#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TilePosition {
    pub x: i32,
    pub y: i32,
}

impl TilePosition {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}
