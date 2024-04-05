use crate::helpers::*;
use crate::resources::{InputEvent, InputQueue, MouseState};
use bevy::prelude::*;

use crate::api::Position;

pub struct InputProcessPlugin;

impl Plugin for InputProcessPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, process_mouse_position.in_set(Phase::InputProcess));
    }
}

fn process_mouse_position(
    mut input_queue: ResMut<InputQueue>,
    mut mouse_state: ResMut<MouseState>,
) {
    for event in input_queue.as_mut().0.drain(..) {
        match event {
            InputEvent::MouseMove(position) => mouse_state.as_mut().position = position,
            InputEvent::MouseClick => todo!(),
        }
    }
}
