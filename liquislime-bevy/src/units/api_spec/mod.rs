use bevy::ecs::world;
use bevy::input::touch::Touch;
use bevy::prelude::*;

use self::helpers::*;

use super::{global_storage::*, UnitId};
use crate::api::*;
use crate::components::FactionComponent;
use crate::resources::MouseState;
use crate::{components::TilePositionComponent, helpers::ResultLogger};

mod helpers;

pub fn level_width() -> i32 {
    get_level_info().width as i32 // TODO: why is level width an i32?
}

pub fn level_height() -> i32 {
    get_level_info().height as i32
}

pub fn get_own_position() -> TilePosition {
    let mut world = get_world();
    let mut query = world.query::<(&UnitId, &TilePositionComponent)>();
    for (unit_id, tile_position) in query.iter(&world) {
        if *unit_id == get_current_unit() {
            return tile_position.0;
        }
    }
    panic!("get_own_position did not find current unit")
}

pub fn get_own_faction() -> Faction {
    let mut world = get_world();
    let mut query = world.query::<(&UnitId, &FactionComponent)>();
    for (unit_id, faction) in query.iter(&world) {
        if *unit_id == get_current_unit() {
            return faction.0;
        }
    }
    panic!("get_own_faction did not find current unit")
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

pub fn get_slime_amount(faction: Faction, position: TilePosition) -> SlimeAmount {
    let mut world = get_world();
    let slime_grid = get_slime_grid(&mut world);
    slime_grid.try_get_amount(faction, position).log_err_or(
        &format!("Getting slime amount out of bounds: {position:?}."),
        SlimeAmount::from_integer(0),
    )
}

pub fn set_slime_amount(faction: Faction, position: TilePosition, amount: SlimeAmount) {
    let mut world = get_world();
    let mut slime_grid = get_slime_grid(&mut world);
    slime_grid
        .try_set_amount(faction, position, amount)
        .log_err(&format!(
            "Setting slime amount out of bounds: {position:?}."
        ));
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
    let mut world = get_world();
    let mouse_state = world.resource::<MouseState>();
    mouse_state.position
}

pub fn is_mouse_pressed() -> bool {
    let mut world = get_world();
    let mouse_state = world.resource::<MouseState>();
    mouse_state.pressed
}
