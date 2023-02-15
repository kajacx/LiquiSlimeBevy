use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TilePosition {
    pub x: i32,
    pub y: i32,
}

impl TilePosition {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn from_floats_floor(x: f32, y: f32) -> Self {
        Self {
            x: x.floor() as i32,
            y: y.floor() as i32,
        }
    }

    pub fn to_centered_vec(self, z: f32) -> Vec3 {
        Vec3::new(self.x as f32 + 0.5, self.y as f32 + 0.5, z)
    }
}