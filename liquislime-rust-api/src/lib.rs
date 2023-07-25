pub(crate) mod protocol {
    wit_bindgen::generate!({
        path: "../protocol.wit",
        world: "liquislime-unit",
        macro_export,
    });
}

#[allow(unused)]
mod types;
pub use types::*;

pub fn get_own_position() -> TilePosition {
    let pos = protocol::get_own_position();
    TilePosition::new(pos.x, pos.y)
}

pub fn get_mouse_touch() -> Option<Position> {
    if protocol::is_mouse_pressed(protocol::MouseInput::Primary) {
        protocol::get_mouse_position().map(Position::from_protocol)
    } else {
        None
    }
}

pub trait LiquislimeUnit {
    fn update(time_elapsed: TimeInterval);
}

impl<T: LiquislimeUnit> protocol::LiquislimeUnit for T {
    fn update(time_elapsed: protocol::TimeInterval) {
        T::update(TimeInterval::from_protocol(time_elapsed))
    }
}

// pub use protocol::export_liquislime_unit;

// #[macro_export]
// macro_rules! export_liquislime_unit_my {
//     ($name: ident) => {
//         $crate::export_liquislime_unit!($name);
//     };
// }

// Target arch will always be wasm32, but this removes Rust Analyzer faulty error
#[cfg(target_arch = "wasm32")]
pub use protocol::__link_section;
pub use protocol::call_update;
