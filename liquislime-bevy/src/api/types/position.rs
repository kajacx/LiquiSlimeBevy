use crate::SlimeAmount;
use derive_more::{Add, AddAssign, Sub, SubAssign};

#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    Add,
    Sub,
    AddAssign,
    SubAssign,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
pub struct Position {
    x: f32,
    y: f32,
}

impl TilePosition {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn from_tile_center(tile_position: TilePosition) -> Self {
        Self::new(tile_position.x as f32 + 0.5, tile_position.y as f32 + 0.5)
    }

    pub fn from_tile_bottom_left(tile_position: TilePosition) -> Self {
        Self::new(tile_position.x as f32, tile_position.y as f32)
    }

    pub fn to_tile_position(self) -> TilePosition {
        TilePosition::from_position(self)
    }
}
