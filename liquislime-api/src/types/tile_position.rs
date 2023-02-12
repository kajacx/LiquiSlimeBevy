use fp_bindgen::prelude::Serializable;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serializable)]
pub struct TilePosition {
    pub x: i32,
    pub y: i32,
}
