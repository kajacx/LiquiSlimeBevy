use fp_bindgen::prelude::Serializable;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serializable)]
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
}
