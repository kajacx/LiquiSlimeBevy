use std::{ops::Deref, sync::Mutex};

use crate::units::api_spec::types::Position;

// TODO: rename to mouse position
#[derive(Clone, Debug)]
pub struct MouseState {
    pub position: Option<Position>,
}

static MOUSE_STATE: Mutex<MouseState> = Mutex::new(MouseState { position: None });

pub fn get_mouse_state() -> impl Deref<Target = MouseState> {
    MOUSE_STATE.lock().expect("Get mouse state mutex lock")
}

pub fn set_mouse_state(state: MouseState) {
    *MOUSE_STATE.lock().expect("Get mouse state mutex lock") = state;
}
