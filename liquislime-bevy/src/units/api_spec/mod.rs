use self::types::*;

use super::{
    global_storage::{get_current_unit, get_world},
    UnitId,
};

pub mod bindings;
pub mod types;

fn level_width() -> i32 {
    10 // TODO
}
fn level_height() -> i32 {
    10 // TODO
}

fn get_own_position() -> TilePosition {
    let query = get_world().query::<(&UnitId, &TilePosition)>();
    for (unit_id, tile_position) in &query {
        if unit_id == get_current_unit() {
            return tile_position.clone(); // TODO: make sure it is copied and not cloned
        }
    }
    panic!("get_own_position did not find current unit")
}

fn set_own_position(position: TilePosition) {
    let mut query = get_world().query::<(&UnitId, &mut TilePosition)>();
    for (unit_id, tile_position) in &mut query {
        if unit_id == get_current_unit() {
            *tile_position = position;
            return;
        }
    }
    // TODO: this needs to be reworked way better
    panic!("set_own_position did not find current unit")
}

fn get_slime_amount(position: TilePosition) -> SlimeAmount {
    SlimeAmount::from_integer(50) // TODO
}
fn set_slime_amount(position: TilePosition, amount: SlimeAmount) {
    // TODO
}

fn was_mouse_just_pressed(button: MouseButton) -> bool {
    false // TODO
}
fn is_mouse_pressed(button: MouseButton) -> bool {
    false // TODO
}
fn was_mouse_just_released(button: MouseButton) -> bool {
    false // TODO
}

fn get_mouse_position() -> Position {
    Position::new(2.5, 5.5) // TODO
}
