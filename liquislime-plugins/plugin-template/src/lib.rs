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
        plugin::LiquislimeUnit::update(TimeInterval::from_protocol(time_elapsed));
    }
}

trait LiquislimePlugin {
    fn update(time_elapsed: TimeInterval);
}
