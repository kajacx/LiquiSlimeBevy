use self::{helpers::get_slime_grid, types::*};

use super::{
    global_storage::{get_current_unit, get_level_info, get_mouse_state, get_world},
    UnitId,
};
use crate::helpers::ResultLogger;

pub mod bindings;
mod helpers;
pub mod types;

fn level_width() -> i32 {
    get_level_info().width as i32 // TODO: why is level width an i32?
}
fn level_height() -> i32 {
    get_level_info().height as i32
}

fn get_own_position() -> TilePosition {
    let mut world = get_world();
    let mut query = world.query::<(&UnitId, &TilePosition)>();
    for (unit_id, tile_position) in query.iter(&world) {
        if *unit_id == get_current_unit() {
            return tile_position.clone(); // TODO: make sure it is copied and not cloned
        }
    }
    // TODO: log as error and return 0,0 position instead?
    panic!("get_own_position did not find current unit")
}
fn set_own_position(position: TilePosition) {
    let mut world = get_world();
    let mut query = world.query::<(&UnitId, &mut TilePosition)>();
    for (unit_id, mut tile_position) in query.iter_mut(&mut world) {
        if *unit_id == get_current_unit() {
            *tile_position = position;
            return;
        }
    }
    // TODO: this needs to be reworked way better
    panic!("set_own_position did not find current unit")
}

fn get_slime_amount(position: TilePosition) -> SlimeAmount {
    let mut world = get_world();
    let slime_grid = get_slime_grid(&mut world);
    slime_grid
        .try_get_amount(position)
        .log_err_or(SlimeAmount::from_integer(0))
    //.clone() // TODO: force copy
}
fn set_slime_amount(position: TilePosition, amount: SlimeAmount) {
    let mut world = get_world();
    let mut slime_grid = get_slime_grid(&mut world);
    slime_grid.try_set_amount(position, amount).log_err();
}

fn was_mouse_just_pressed(button: MouseButton) -> bool {
    get_mouse_state().just_pressed
}
fn is_mouse_pressed(button: MouseButton) -> bool {
    get_mouse_state().pressed
}
fn was_mouse_just_released(button: MouseButton) -> bool {
    get_mouse_state().just_released
}

fn get_mouse_position() -> Option<Position> {
    get_mouse_state().position
}
