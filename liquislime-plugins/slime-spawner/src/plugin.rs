use super::*;

pub struct LiquislimeUnit;

impl LiquislimePlugin for LiquislimeUnit {
    fn update(time_elapsed: TimeInterval) {
        let amount = SlimeAmount::from_integer(100);
        TilePosition::own_position().set_own_slime_amount_at_least(amount);

        if let Some(position) = Mouse::is_pressed_in_bounds_at() {
            position
                .to_tile_position()
                .add_own_slime_amount(amount * time_elapsed.to_seconds() * 20);
        }
    }
}
