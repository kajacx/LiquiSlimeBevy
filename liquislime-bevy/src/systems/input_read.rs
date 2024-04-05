use crate::helpers::*;
use crate::resources::{InputEvent, InputQueue};
use bevy::input::touch::Touch;
use bevy::prelude::*;

use crate::api::Position;

pub struct InputReadPlugin;

impl Plugin for InputReadPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_mouse_position.in_set(Phase::InputRead));
        app.add_systems(Update, update_mouse_click.in_set(Phase::InputRead));
    }
}

fn update_mouse_position(
    mut input_queue: ResMut<InputQueue>,
    primary_window: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    touches: Res<Touches>,
) {
    let (camera, transform) = camera_query.single();
    let window = primary_window.single();

    let window_position = window
        .cursor_position()
        .or_else(|| touches.iter().next().map(Touch::position));

    let position = window_position.map(|window_position| {
        window_position_to_world_position(window_position, window, camera, transform)
    });

    if let Some(position) = position {
        input_queue.as_mut().0.push(InputEvent::MouseMove(position));
    }
}

fn update_mouse_click(
    mut input_queue: ResMut<InputQueue>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
) {
    if mouse_input.pressed(MouseButton::Left) || touches.iter().next().is_some() {
        input_queue.0.push(InputEvent::MousePressed)
    }

    // TODO: just pressed and released for touches
    if mouse_input.just_pressed(MouseButton::Left) {
        input_queue.0.push(InputEvent::MouseJustPressed)
    }

    if mouse_input.just_released(MouseButton::Left) {
        input_queue.0.push(InputEvent::MouseJustReleased)
    }
}
