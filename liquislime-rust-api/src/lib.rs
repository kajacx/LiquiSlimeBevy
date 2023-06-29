use std::sync::Mutex;

mod protocol {
    wit_bindgen::generate!({
        path: "../protocol.wit",
        world: "liquislime-unit",
        // macro_export,
    });
}

pub struct TimeInterval(protocol::TimeInterval);

pub struct TilePosition(protocol::TilePosition);

pub struct SlimeAmount(protocol::SlimeAmount);

pub fn get_own_position() -> TilePosition {
    TilePosition(protocol::get_own_position())
}

pub fn add_slime_amount(position: TilePosition, amount: SlimeAmount) {
    protocol::add_slime_amount(position.0, amount.0)
}

pub trait LiquislimeUnit: Send {
    fn update(&self, time_elapsed: TimeInterval);
}

impl<T: LiquislimeUnit> protocol::LiquislimeUnit for T {
    fn update(&self, time_elapsed: protocol::TimeInterval) {
        T::update(TimeInterval(time_elapsed))
    }
}

struct UnitHolder;

static UNIT_HOLDER: Mutex<Option<Box<dyn LiquislimeUnit>>> = Mutex::new(Option::None);

// pub use protocol::export_liquislime_unit;

// #[macro_export]
// macro_rules! export_liquislime_unit_my {
//     ($name: ident) => {
//         $crate::export_liquislime_unit!($name);
//     };
// }
