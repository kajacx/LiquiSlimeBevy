use super::*;

pub struct Level;

impl Level {
    pub fn width() -> i32 {
        crate::protocol::level_width()
    }

    pub fn height() -> i32 {
        crate::protocol::level_height()
    }
}
