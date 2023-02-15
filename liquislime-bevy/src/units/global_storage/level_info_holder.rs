use std::{ops::Deref, sync::Mutex};

#[derive(Clone, Debug)]
pub struct LevelInfo {
    pub width: usize,
    pub height: usize,
}

static LEVEL_INFO: Mutex<LevelInfo> = Mutex::new(LevelInfo {
    width: 10,
    height: 10,
});

pub fn get_level_info() -> impl Deref<Target = LevelInfo> {
    LEVEL_INFO.lock().expect("Get level info mutex lock")
}
