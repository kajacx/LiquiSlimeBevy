use bevy::ecs::world::World;
use std::{
    ops::{Deref, DerefMut},
    sync::Mutex,
};

// TODO: This could be an atomic cell or something?
pub static CURRENT_UNIT: Mutex<Option<UnitId>> = Mutex::new(None);

pub fn set_current_unit(unit: UnitId) {
    (*CURRENT_UNIT.lock().expect("Set current unit mutex lock")) = Some(unit);
}

pub fn get_current_unit() -> UnitId {
    (*CURRENT_UNIT.lock().expect("Get current unit mutex lock"))
        .expect("Current unit should have been set")
}
