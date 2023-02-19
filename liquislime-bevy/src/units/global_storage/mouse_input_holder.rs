use bevy::{
    ecs::world::World,
    prelude::{Input, MouseButton},
};
use std::{
    ops::{Deref, DerefMut},
    ptr,
    sync::Mutex,
};

static MOUSE_INPUT: Mutex<*const Input<MouseButton>> = Mutex::new(ptr::null());

pub fn set_mouse_input(mouse_input: &Input<MouseButton>) {
    (*MOUSE_INPUT.lock().expect("Set mouse input mutex lock")).0 = mouse_input as *const _;
}

pub fn get_mouse_input() -> impl Deref<Target = Input<MouseButton>> {
    // TODO: Horribly unsafe how to ensure safety more properly?
    let locked = MOUSE_INPUT.lock().expect("Get mouse input mutex lock");
    let inner = locked.0;
    unsafe { &*inner }
}

struct UnsafeMouseInputRef(*const Input<MouseButton>);
//unsafe impl Send for UnsafeWorldRef {}
