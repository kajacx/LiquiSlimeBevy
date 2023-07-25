use crate::helpers::*;
use bevy::prelude::*;

use crate::units::global_storage::{set_mouse_state, MouseState};

use crate::api::Position;

pub struct GameInputPlugin;

impl Plugin for GameInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_mouse_position.in_set(Phase::InputRead));
    }
}

fn update_mouse_position(
    primary_window: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let camera_pair = match camera_query.get_single() {
        Ok(camera) => camera,
        Err(err) => {
            warn!("Oh no, camera not found: {:?}", err);
            return;
        }
    };

    let (camera, camera_transform) = camera_pair;

    // get the window that the camera is displaying to (or the primary window)
    let window = primary_window.single();

    set_mouse_state(MouseState {
        position: get_mouse_position(window, camera, camera_transform),
    });
}

fn get_mouse_position(
    window: &Window,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> Option<Position> {
    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = window.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(window.width() as f32, window.height() as f32);

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        let world_pos: Vec2 = world_pos.truncate();

        Some(Position {
            x: world_pos.x,
            y: world_pos.y,
        })
    } else {
        None
    }
}
