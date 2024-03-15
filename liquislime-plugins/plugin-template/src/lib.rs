mod protocol {
    use super::*;

    wit_bindgen::generate!({
        path: "../../protocol.wit",
        world: "liquislime-unit",
        exports: {
           world: LiquislimeWorld
        }
    });
}

mod plugin;
#[allow(unused)]
mod types;

use types::*;

struct LiquislimeWorld;

impl protocol::Guest for LiquislimeWorld {
    fn update(time_elapsed: protocol::TimeInterval) {
        let time_elapsed = TimeInterval::from_protocol(time_elapsed);
        plugin::LiquislimeUnit::update(time_elapsed);
    }
}

trait LiquislimePlugin {
    fn update(time_elapsed: TimeInterval);
}

#[allow(unused)]
fn get_own_position() -> TilePosition {
    let position = protocol::get_own_position();
    TilePosition::from_protocol(position)
}

#[allow(unused)]
fn get_mouse_position() -> Option<Position> {
    let position = protocol::get_mouse_position();
    position.map(Position::from_protocol)
}

#[allow(unused)]
fn get_mouse_position_in_bounds() -> Option<Position> {
    get_mouse_position().filter(|p| Position::is_in_bounds(*p))
}

#[allow(unused)]
fn get_mouse_touch() -> Option<Position> {
    get_mouse_position().filter(|_| protocol::is_mouse_pressed(protocol::MouseInput::Primary))
}

#[allow(unused)]
fn get_mouse_touch_in_bounds() -> Option<Position> {
    get_mouse_touch().filter(|p| Position::is_in_bounds(*p))
}
