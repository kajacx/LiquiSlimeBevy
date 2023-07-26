use bevy::prelude::*;

use crate::{
    api::Position,
    units::global_storage::{set_mouse_state, MouseState},
};

pub fn set_mouse_position_from_window_position(
    window_position: Vec2,
    window: &Window,
    camera: &Camera,
    transform: &GlobalTransform,
) {
    // get the size of the window
    let window_size = Vec2::new(window.width() as f32, window.height() as f32);

    // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
    let mut ndc = (window_position / window_size) * 2.0 - Vec2::ONE;
    ndc.y = -ndc.y; // TODO: Y got flipped, investigate

    // matrix for undoing the projection and camera transform
    let ndc_to_world = transform.compute_matrix() * camera.projection_matrix().inverse();

    // use it to convert ndc to world-space coordinates
    let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

    // reduce it to a 2D value
    let world_pos: Vec2 = world_pos.truncate();

    // convert to Position
    let position = Some(Position {
        x: world_pos.x,
        y: world_pos.y,
    });

    set_mouse_state(MouseState { position });
}
