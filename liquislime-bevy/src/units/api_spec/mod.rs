use bevy::input::touch::Touch;
use bevy::prelude::*;

use self::helpers::*;

use super::{global_storage::*, UnitId};
use crate::api::*;
use crate::helpers::set_mouse_position_from_window_position;
use crate::{components::TilePositionComponent, helpers::ResultLogger};

mod helpers;

fn level_width() -> i32 {
    get_level_info().width as i32 // TODO: why is level width an i32?
}
fn level_height() -> i32 {
    get_level_info().height as i32
}

pub fn get_own_position() -> TilePosition {
    let mut world = get_world();
    let mut query = world.query::<(&UnitId, &TilePositionComponent)>();
    for (unit_id, tile_position) in query.iter(&world) {
        if *unit_id == get_current_unit() {
            return tile_position.0.clone(); // TODO: make sure it is copied and not cloned
        }
    }
    // TODO: log as error and return 0,0 position instead?
    panic!("get_own_position did not find current unit")
}
// fn set_own_position(position: TilePosition) {
//     let mut world = get_world();
//     let mut query = world.query::<(&UnitId, &mut TilePositionComponent)>();
//     for (unit_id, mut tile_position) in query.iter_mut(&mut world) {
//         if *unit_id == get_current_unit() {
//             tile_position.0 = position;
//             return;
//         }
//     }
//     // TODO: this needs to be reworked way better
//     panic!("set_own_position did not find current unit")
// }

// fn get_slime_amount(position: TilePosition) -> SlimeAmount {
//     let mut world = get_world();
//     let slime_grid = get_slime_grid(&mut world);
//     slime_grid
//         .try_get_amount(position)
//         .log_err_or(SlimeAmount::from_integer(0))
// }
// fn set_slime_amount(position: TilePosition, amount: SlimeAmount) {
//     let mut world = get_world();
//     let mut slime_grid = get_slime_grid(&mut world);
//     slime_grid.try_set_amount(position, amount).log_err();
// }
pub fn add_slime_amount(
    faction: Faction,
    position: TilePosition,
    amount: SlimeAmount,
) -> SlimeAmount {
    let mut world = get_world();
    let mut slime_grid = get_slime_grid(&mut world);
    let curr_amount = slime_grid.try_get_amount(faction.index(), position);
    if let Some(curr) = curr_amount {
        let new_amount = (curr + amount).non_negative();
        slime_grid
            .try_set_amount(faction.index(), position, new_amount)
            .expect("We have checked if position is in range");
        new_amount
    } else {
        SlimeAmount::from_integer(0)
    }
}

// fn is_mouse_pressed(mouse_button: MouseButton) -> bool {
//     read_mouse_input(|input| input.pressed(api_mouse_button_to_bevy(mouse_button)))
// }
// fn was_mouse_just_pressed(mouse_button: MouseButton) -> bool {
//     read_mouse_input(|input| input.just_pressed(api_mouse_button_to_bevy(mouse_button)))
// }
// fn was_mouse_just_released(mouse_button: MouseButton) -> bool {
//     read_mouse_input(|input| input.just_released(api_mouse_button_to_bevy(mouse_button)))
// }

pub fn get_mouse_position() -> Option<Position> {
    get_mouse_state().position
}

pub fn is_mouse_pressed() -> bool {
    let mut world = get_world();

    let input = world
        .get_resource::<Input<MouseButton>>()
        .expect("Mouse input resource should exist");

    let pressed = input.pressed(MouseButton::Left);
    let mut touch_window_position = Option::<Vec2>::None;

    let touches = world
        .get_resource::<Touches>()
        .expect("Touches resource should exist");
    for touch in touches.iter() {
        info!("Hello touch: {:?}", touch);
        //touch.
        // touch.position()
        info!(
            "Hello TOUCH WINDOW POSITION: {:?}",
            get_touch_window_position(touch)
        );
        touch_window_position = Some(get_touch_window_position(touch));
    }

    if let Some(window_position) = touch_window_position {
        let mut window = world.query::<&Window>();
        let mut camera_and_transform = world.query::<(&Camera, &GlobalTransform)>();

        let window = window.single(&world);
        let (camera, transform) = camera_and_transform.single(&world);

        set_mouse_position_from_window_position(window_position, window, camera, transform);
    }

    return pressed || touch_window_position.is_some();
}

fn get_touch_window_position(touch: &Touch) -> Vec2 {
    let global_position = touch.position();

    let window_x = unsafe {
        js_sys::eval(&format!(
            "document.getElementById('{}').getBoundingClientRect().x",
            crate::RENDER_CANVAS_ID
        ))
    }
    .unwrap()
    .as_f64()
    .unwrap();

    let window_y = unsafe {
        js_sys::eval(&format!(
            "document.getElementById('{}').getBoundingClientRect().y",
            crate::RENDER_CANVAS_ID
        ))
    }
    .unwrap()
    .as_f64()
    .unwrap();

    Vec2::new(
        global_position.x - window_x as f32,
        global_position.y - window_y as f32,
    )
}
