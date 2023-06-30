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
pub struct TilePosition {
    x: i32,
    y: i32,
}

impl TilePosition {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn add_slime_amount(self, amount: SlimeAmount) {
        crate::protocol::add_slime_amount(self.as_protocol(), amount.as_protocol());
    }

    pub(crate) fn as_protocol(self) -> crate::protocol::TilePosition {
        crate::protocol::TilePosition {
            x: self.x,
            y: self.y,
        }
    }

    pub(crate) fn from_protocol(position: crate::protocol::TilePosition) -> Self {
        Self {
            x: position.x,
            y: position.y,
        }
    }

    // pub fn from_position(position: Position) -> Self {
    //     Self::new(position.x.floor() as i32, position.y.floor() as i32)
    // }

    // pub fn to_position_center(self) -> Position {
    //     Position::from_tile_center(self)
    // }

    // pub fn to_position_bottom_left(self) -> Position {
    //     Position::from_tile_bottom_left(self)
    // }
}
