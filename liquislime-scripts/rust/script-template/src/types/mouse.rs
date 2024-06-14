use super::Position;
use crate::api::FromWasmAbi;

pub struct Mouse;

impl Mouse {
    pub fn position() -> Position {
        // TODO: get last position instead
        Self::try_get_position().unwrap_or_default()
    }

    pub fn try_get_position() -> Option<Position> {
        Option::<Position>::from_wasm_abi(unsafe { crate::api::get_mouse_position() }).unwrap()
    }

    pub fn try_get_position_in_bounds() -> Option<Position> {
        Self::try_get_position().filter(|pos| Position::is_in_bounds(*pos))
    }

    pub fn is_pressed() -> bool {
        bool::from_wasm_abi(unsafe { crate::api::is_mouse_pressed() }).unwrap()
    }

    pub fn is_pressed_at() -> Option<Position> {
        Self::try_get_position().filter(|_| Self::is_pressed())
    }

    pub fn is_pressed_in_bounds_at() -> Option<Position> {
        Self::try_get_position_in_bounds().filter(|_| Self::is_pressed())
    }
}
