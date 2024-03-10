use std::{ops::Deref, sync::Mutex};

use crate::{WORLD_HEIGHT, WORLD_WIDTH};

#[derive(Clone, Debug)]
pub struct LevelInfo {
    pub width: usize,
    pub height: usize,
}

static LEVEL_INFO: Mutex<LevelInfo> = Mutex::new(LevelInfo {
    width: WORLD_WIDTH,
    height: WORLD_HEIGHT,
});

pub fn get_level_info() -> impl Deref<Target = LevelInfo> {
    LEVEL_INFO.lock().expect("Get level info mutex lock")
}
