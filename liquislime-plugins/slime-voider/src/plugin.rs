use super::*;

pub struct LiquislimeUnit;

impl LiquislimePlugin for LiquislimeUnit {
    fn update(_time_elapsed: TimeInterval) {
        let amount = SlimeAmount::from_integer(150);
        get_own_position().set_own_slime_amount_at_least(amount);
    }
}
