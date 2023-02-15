use std::{
    ops::{Deref, DerefMut},
    sync::Mutex,
};

use bevy::prelude::DerefMut;

pub static GLOBAL_WORLD: Mutex<*mut World> = Mutex::new(std::ptr::null_mut());

pub fn set_world(world: &mut World) {
    *GLOBAL_WORLD.lock() = world as *mut _;
}

pub fn get_world() -> impl DerefMut<Target = World> {
    // TODO: Horribly unsafe how to ensure safety more properly?
    unsafe { *GLOBAL_WORLD.lock() as &mut World }
}
