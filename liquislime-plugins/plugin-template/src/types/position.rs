use super::*;
use derive_more::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, Clone, Copy, Default, Add, Sub, AddAssign, SubAssign, PartialEq, PartialOrd)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn add_x(self, x_add: f32) -> Self {
        Self {
            x: self.x + x_add,
            ..self
        }
    }

    pub fn add_y(self, y_add: f32) -> Self {
        Self {
            y: self.y + y_add,
            ..self
        }
    }

    pub fn from_tile_center(tile_position: TilePosition) -> Self {
        Self::new(tile_position.x as f32 + 0.5, tile_position.y as f32 + 0.5)
    }

    pub const fn from_tile_bottom_left(tile_position: TilePosition) -> Self {
        Self::new(tile_position.x as f32, tile_position.y as f32)
    }

    pub fn to_tile_position(self) -> TilePosition {
        TilePosition::from_position(self)
    }

    pub fn is_in_bounds(self) -> bool {
        self.to_tile_position().is_in_bounds()
    }

    pub fn is_in_tile(self, tile_position: TilePosition) -> bool {
        tile_position.contains(self)
    }

    pub fn as_protocol(self) -> crate::protocol::Position {
        crate::protocol::Position {
            x: self.x,
            y: self.y,
        }
    }

    pub fn from_protocol(position: crate::protocol::Position) -> Self {
        Self {
            x: position.x,
            y: position.y,
        }
    }
}
