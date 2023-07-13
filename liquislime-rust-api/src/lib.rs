pub(crate) mod protocol {
    wit_bindgen::generate!({
        path: "../protocol.wit",
        world: "liquislime-unit",
        macro_export,
    });
}

mod types;
pub use types::*;

pub fn get_own_position() -> TilePosition {
    let pos = protocol::get_own_position();
    TilePosition::new(pos.x, pos.y)
}

// pub fn add_slime_amount(position: TilePosition, amount: SlimeAmount) {
//     protocol::add_slime_amount(position.0, amount.0)
// }

pub trait LiquislimeUnit {
    fn update(time_elapsed: TimeInterval);
}

impl<T: LiquislimeUnit> protocol::LiquislimeUnit for T {
    fn update(time_elapsed: protocol::TimeInterval) {
        T::update(TimeInterval::from_protocol(time_elapsed))
    }
}

// struct UnitHolder;

// static UNIT_HOLDER: Mutex<Option<Box<dyn LiquislimeUnit>>> = Mutex::new(Option::None);

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
