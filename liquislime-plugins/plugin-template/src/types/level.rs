use super::*;

pub struct Level;

impl Level {
    pub fn width() -> i32 {
        crate::level_width()
    }

    pub fn height() -> i32 {
        crate::level_height()
    }
}
