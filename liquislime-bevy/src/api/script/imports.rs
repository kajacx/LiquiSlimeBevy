use super::LiquislimeImports;
use crate::api::{ApiFaction, ApiPosition, ApiSlimeAmount, ApiTilePosition, ApiUnit};
use crate::components::{FactionComponent, UnitId};
use crate::resources::MouseState;
use crate::units::api_spec::get_slime_grid;
use crate::units::global_storage::{
    get_current_instance, get_current_unit, get_level_info, get_world,
};
use crate::{components::TilePositionComponent, helpers::ResultLogger};
use bevy::ecs::world;
use bevy::input::touch::Touch;
use bevy::prelude::*;

const LOG: bool = false;

#[derive(Debug, Clone)]
pub struct GameImports;

impl LiquislimeImports for GameImports {
    fn level_width(&self) -> i32 {
        if LOG {
            println!("Entered level_width")
        }
        level_width()
    }

    fn level_height(&self) -> i32 {
        if LOG {
            println!("Entered level_height")
        }
        level_height()
    }

    fn get_current_unit(&self) -> ApiUnit {
        if LOG {
            println!("Entered get_current_unit")
        }
        ApiUnit(get_current_unit())
    }

    fn get_current_instance(&self) -> crate::api::ApiInstance {
        if LOG {
            println!("Entered get_current_instance")
        }
        get_current_instance()
    }

    fn get_own_faction(&self) -> ApiFaction {
        if LOG {
            println!("Entered get_own_faction")
        }
        get_own_faction()
    }

    fn get_own_position(&self) -> ApiTilePosition {
        if LOG {
            println!("Entered get_own_position")
        }
        get_own_position()
    }

    fn get_slime_amount(&self, faction: ApiFaction, position: ApiTilePosition) -> ApiSlimeAmount {
        if LOG {
            println!("Entered get_slime_amount")
        }
        get_slime_amount(faction, position)
    }

    fn set_slime_amount(
        &self,
        faction: ApiFaction,
        position: ApiTilePosition,
        amount: ApiSlimeAmount,
    ) {
        if LOG {
            println!("Entered set_slime_amount")
        }
        set_slime_amount(faction, position, amount)
    }

    fn get_mouse_position(&self) -> Option<ApiPosition> {
        if LOG {
            println!("Entered is_mouse_pressed")
        }
        get_mouse_position()
    }

    fn is_mouse_pressed(&self) -> bool {
        if LOG {
            println!("Entered is_mouse_pressed")
        }
        is_mouse_pressed()
    }

    fn log(&self, message: &str) {
        // TODO: Script name, unit id, etc.
        println!("Script says: {}", message)
    }
}

fn level_width() -> i32 {
    get_level_info().width as i32 // TODO: why is level width an i32?
}

fn level_height() -> i32 {
    get_level_info().height as i32
}

fn get_own_position() -> ApiTilePosition {
    let mut world = get_world();
    let mut query = world.query::<(&UnitId, &TilePositionComponent)>();
    for (unit_id, tile_position) in query.iter(&world) {
        if *unit_id == get_current_unit() {
            return tile_position.0;
        }
    }
    panic!("get_own_position did not find current unit")
}

fn get_own_faction() -> ApiFaction {
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

fn get_slime_amount(faction: ApiFaction, position: ApiTilePosition) -> ApiSlimeAmount {
    let mut world = get_world();
    let slime_grid = get_slime_grid(&mut world);
    slime_grid.try_get_amount(faction, position).log_err_or(
        &format!("Getting slime amount out of bounds: {position:?}."),
        ApiSlimeAmount::from_integer(0),
    )
}

fn set_slime_amount(faction: ApiFaction, position: ApiTilePosition, amount: ApiSlimeAmount) {
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

fn get_mouse_position() -> Option<ApiPosition> {
    let mut world = get_world();
    let mouse_state = world.resource::<MouseState>();
    mouse_state.position
}

fn is_mouse_pressed() -> bool {
    let mut world = get_world();
    let mouse_state = world.resource::<MouseState>();
    mouse_state.pressed
}
