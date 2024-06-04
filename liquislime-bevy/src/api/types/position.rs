use super::ApiTilePosition;
use bevy::prelude::Vec3;
use derive_more::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, Clone, Copy, Default, Add, Sub, AddAssign, SubAssign, PartialEq, PartialOrd)]
pub struct ApiPosition {
    pub x: f32,
    pub y: f32,
}

impl ApiPosition {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn from_tile_center(tile_position: ApiTilePosition) -> Self {
        Self::new(tile_position.x as f32 + 0.5, tile_position.y as f32 + 0.5)
    }

    pub fn from_tile_bottom_left(tile_position: ApiTilePosition) -> Self {
        Self::new(tile_position.x as f32, tile_position.y as f32)
    }

    pub fn to_tile_position(self) -> ApiTilePosition {
        ApiTilePosition::from_position(self)
    }

    pub fn is_in_tile(self, tile_position: ApiTilePosition) -> bool {
        tile_position.contains(self)
    }

    pub fn to_vec3(self, z: f32) -> Vec3 {
        Vec3::new(self.x, self.y, z)
    }
}
