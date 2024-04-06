use crate::components::TilePositionComponent;
use crate::helpers::*;
use crate::resources::{InputEvent, InputQueue, MouseState, SelectedUnit};
use crate::units::UnitId;
use bevy::prelude::*;

use crate::api::{Position, TilePosition};

pub struct InputProcessPlugin;

impl Plugin for InputProcessPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, process_mouse_position.in_set(Phase::InputProcess));
    }
}

fn process_mouse_position(
    mut input_queue: ResMut<InputQueue>,
    mut mouse_state: ResMut<MouseState>,
    mut selected_unit: ResMut<SelectedUnit>,
    units: Query<(&TilePositionComponent, &UnitId)>,
) {
    // Reset mouse state
    *mouse_state.as_mut() = MouseState::default();

    // Fill it from events
    for event in input_queue.as_mut().0.drain(..) {
        match event {
            InputEvent::MouseMove(position) => mouse_state.as_mut().position = Some(position),
            InputEvent::MousePressed => mouse_state.as_mut().pressed = true,
            InputEvent::MouseJustPressed => mouse_state.as_mut().just_pressed = true,
            InputEvent::MouseJustReleased => mouse_state.as_mut().just_released = true,
        }
    }

    // Select a unit
    if mouse_state.just_pressed {
        selected_unit.0 = mouse_state.position.and_then(|position| {
            units.iter().find_map(|(tile_position, id)| {
                if position.is_in_tile(tile_position.0) {
                    Some(*id)
                } else {
                    None
                }
            })
        });
    }
}
