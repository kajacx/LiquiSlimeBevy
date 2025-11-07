use glam::{IVec2, Vec2};

pub type Position = Vec2;
pub type TilePosition = IVec2;

mod faction;
mod game_state;
mod slime_amount;
mod slime_grid;
mod slime_grids;
mod time_interval;

pub use faction::*;
pub use game_state::*;
pub use slime_amount::*;
pub use slime_grid::*;
pub use slime_grids::*;
pub use time_interval::*;
