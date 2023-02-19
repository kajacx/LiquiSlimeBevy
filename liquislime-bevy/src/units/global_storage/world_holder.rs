use bevy::{ecs::world::World, prelude::*};
use std::{borrow::Borrow, ops::DerefMut, sync::Mutex};

static GLOBAL_WORLD: Mutex<UnsafeWorldRef> = Mutex::new(UnsafeWorldRef(std::ptr::null_mut()));

pub fn set_world(world: &mut World) {
    (*GLOBAL_WORLD.lock().expect("Set world mutex lock")).0 = world as *mut _;
}

pub fn get_world() -> impl DerefMut<Target = World> {
    // TODO: Horribly unsafe how to ensure safety more properly?
    let locked = GLOBAL_WORLD.lock().expect("Get world mutex lock");
    let inner = locked.0;
    unsafe { &mut *inner }
}

pub fn read_mouse_input<T>(reader: impl FnOnce(&Input<MouseButton>) -> T) -> T {
    let world = get_world();

    let input = world
        .get_resource::<Input<MouseButton>>()
        .expect("Mouse input resource should exist");

    reader(input.borrow())
}

struct UnsafeWorldRef(*mut World);
unsafe impl Send for UnsafeWorldRef {}
