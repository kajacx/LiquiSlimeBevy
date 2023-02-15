use self::types::*;

pub mod bindings;
pub mod types;
pub mod world_holder;

fn level_width() -> i32 {
    10 // TODO
}
fn level_height() -> i32 {
    10 // TODO
}

fn get_own_position() -> TilePosition;
fn set_own_position(position: TilePosition);

fn get_slime_amount(position: TilePosition) -> SlimeAmount;
fn set_slime_amount(position: TilePosition, amount: SlimeAmount);

fn was_mouse_just_pressed(button: MouseButton) -> bool;
fn is_mouse_pressed(button: MouseButton) -> bool;
fn was_mouse_just_released(button: MouseButton) -> bool;

fn get_mouse_position() -> Position;
