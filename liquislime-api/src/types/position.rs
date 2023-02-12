use fp_bindgen::prelude::Serializable;

use super::TilePosition;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, Serializable)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}
