mod protocol {
    wit_bindgen::generate!({
        path: "../protocol.wit",
        world: "liquislime-unit",
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

pub trait LiquislimeUnit {
    fn update(time_elapsed: TimeInterval);
}

impl<T: LiquislimeUnit> protocol::LiquislimeUnit for T {
    fn update(time_elapsed: protocol::TimeInterval) {
        T::update(TimeInterval(time_elapsed))
    }
}
