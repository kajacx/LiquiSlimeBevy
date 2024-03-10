use liquislime_api::*;

struct Voider;

impl LiquislimeUnit for Voider {
    fn update(time_elapsed: TimeInterval) {
        let added_amount_per_second = SlimeAmount::from_integer(2000);
        let added_amount = added_amount_per_second * time_elapsed.to_seconds();
        get_own_position().add_slime_amount(Faction::faction1(), added_amount);
    }
}

export_liquislime_unit!(Voider);
