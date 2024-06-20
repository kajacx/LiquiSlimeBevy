pub struct Level;

impl Level {
    pub fn width() -> i32 {
        unsafe { crate::api::level_width() }
    }

    pub fn height() -> i32 {
        unsafe { crate::api::level_height() }
    }
}
