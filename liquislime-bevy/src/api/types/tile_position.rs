use super::ApiPosition;
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
pub struct ApiTilePosition {
    pub x: i32,
    pub y: i32,
}

impl ApiTilePosition {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn from_position(position: ApiPosition) -> Self {
        Self::new(position.x.floor() as i32, position.y.floor() as i32)
    }

    pub fn to_position_center(self) -> ApiPosition {
        ApiPosition::from_tile_center(self)
    }

    pub fn to_position_bottom_left(self) -> ApiPosition {
        ApiPosition::from_tile_bottom_left(self)
    }

    pub fn contains(self, position: ApiPosition) -> bool {
        position.to_tile_position() == self
    }
}
