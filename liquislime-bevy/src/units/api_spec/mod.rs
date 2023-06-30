use self::helpers::*;

use super::{global_storage::*, UnitId};
use crate::api::*;
use crate::{components::TilePositionComponent, helpers::ResultLogger};

mod helpers;

fn level_width() -> i32 {
    get_level_info().width as i32 // TODO: why is level width an i32?
}
fn level_height() -> i32 {
    get_level_info().height as i32
}

fn get_own_position() -> TilePosition {
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
fn add_slime_amount(position: TilePosition, amount: SlimeAmount) -> SlimeAmount {
    println!("Adding slime amount: {:?}", amount);

    let mut world = get_world();
    let mut slime_grid = get_slime_grid(&mut world);
    let curr_amount = slime_grid.try_get_amount(position);
    if let Some(curr) = curr_amount {
        let new_amount = (curr + amount).non_negative();
        slime_grid
            .try_set_amount(position, new_amount)
            .expect("We have checked if position is in range");
        new_amount
    } else {
        // SlimeAmount::from_integer(0)
        SlimeAmount { amount: 0 }
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

// fn get_mouse_position() -> Option<Position> {
//     get_mouse_state().position
// }
