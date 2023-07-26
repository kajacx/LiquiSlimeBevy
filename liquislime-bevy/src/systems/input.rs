use crate::helpers::*;
use bevy::prelude::*;

use crate::api::Position;
use crate::helpers::set_mouse_position_from_window_position;
use crate::units::global_storage::{set_mouse_state, MouseState};

pub struct GameInputPlugin;

impl Plugin for GameInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_mouse_position.in_set(Phase::InputRead));
    }
}

fn update_mouse_position(
    // this cannot be a single query
    primary_window: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    let (camera, transform) = camera_query.single();
    let window = primary_window.single();

    match window.cursor_position() {
        Some(mouse_position) => {
            set_mouse_position_from_window_position(mouse_position, window, camera, transform)
        }
        None => set_mouse_state(MouseState { position: None }),
    }
}
