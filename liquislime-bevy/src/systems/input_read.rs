use crate::helpers::*;
use crate::resources::{InputEvent, InputQueue};
use bevy::prelude::*;

use crate::api::Position;

pub struct InputReadPlugin;

impl Plugin for InputReadPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_mouse_position.in_set(Phase::InputRead));
    }
}

fn update_mouse_position(
    mut input_queue: ResMut<InputQueue>,
    primary_window: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    let (camera, transform) = camera_query.single();
    let window = primary_window.single();

    let position = window.cursor_position().map(|window_position| {
        window_position_to_world_position(window_position, window, camera, transform)
    });

    input_queue.as_mut().0.push(InputEvent::MouseMove(position));
}
