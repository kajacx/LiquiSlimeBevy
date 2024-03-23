use super::*;
use derive_more::{Add, AddAssign, Sub, SubAssign};

#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    Add,
    Sub,
    AddAssign,
    SubAssign,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
pub struct TilePosition {
    pub x: i32,
    pub y: i32,
}

impl TilePosition {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn own_position() -> Self {
        Self::from_protocol(crate::protocol::get_own_position())
    }

    pub fn is_in_bounds(self) -> bool {
        self.x >= 0 && self.y >= 0 && self.x < Level::width() && self.y < Level::height()
    }

    pub fn get_own_slime_amount(self) -> SlimeAmount {
        self.get_slime_amount(Faction::get_own_faction())
    }

    pub fn get_slime_amount(self, faction: Faction) -> SlimeAmount {
        SlimeAmount::from_protocol(crate::protocol::get_slime_amount(
            faction.as_protocol(),
            self.as_protocol(),
        ))
    }

    pub fn set_own_slime_amount(self, amount: SlimeAmount) {
        self.set_slime_amount(Faction::get_own_faction(), amount)
    }

    pub fn set_slime_amount(self, faction: Faction, amount: SlimeAmount) {
        crate::protocol::set_slime_amount(
            faction.as_protocol(),
            self.as_protocol(),
            amount.as_protocol(),
        )
    }

    pub fn set_own_slime_amount_at_least(self, amount: SlimeAmount) {
        self.set_slime_amount_at_least(Faction::get_own_faction(), amount)
    }

    pub fn set_slime_amount_at_least(self, faction: Faction, amount: SlimeAmount) {
        let amount = SlimeAmount::max(self.get_slime_amount(faction), amount);
        crate::protocol::set_slime_amount(
            faction.as_protocol(),
            self.as_protocol(),
            amount.as_protocol(),
        )
    }

    pub fn add_own_slime_amount(self, amount: SlimeAmount) {
        self.add_slime_amount(Faction::get_own_faction(), amount)
    }

    pub fn add_slime_amount(self, faction: Faction, amount: SlimeAmount) {
        let amount = self.get_slime_amount(faction) + amount;
        crate::protocol::set_slime_amount(
            faction.as_protocol(),
            self.as_protocol(),
            amount.as_protocol(),
        );
    }

    pub const fn add_x(self, x_add: i32) -> Self {
        Self {
            x: self.x + x_add,
            ..self
        }
    }

    pub const fn add_y(self, y_add: i32) -> Self {
        Self {
            y: self.y + y_add,
            ..self
        }
    }

    pub fn from_position(position: Position) -> Self {
        Self::new(position.x.floor() as i32, position.y.floor() as i32)
    }

    pub fn to_position_center(self) -> Position {
        Position::from_tile_center(self)
    }

    pub fn to_position_bottom_left(self) -> Position {
        Position::from_tile_bottom_left(self)
    }

    pub fn as_protocol(self) -> crate::protocol::TilePosition {
        crate::protocol::TilePosition {
            x: self.x,
            y: self.y,
        }
    }

    pub fn from_protocol(position: crate::protocol::TilePosition) -> Self {
        Self {
            x: position.x,
            y: position.y,
        }
    }
}
