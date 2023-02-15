use self::types::*;

pub mod bindings;
pub mod types;

fn level_width() -> i32 {
    10 // TODO
}
fn level_height() -> i32 {
    10 // TODO
}

fn get_own_position() -> TilePosition {
    TilePosition::new(2, 5) // TODO
}
fn set_own_position(position: TilePosition) {
    // TODO
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
