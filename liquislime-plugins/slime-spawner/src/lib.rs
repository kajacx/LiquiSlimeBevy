use liquislime_api::*;

struct Spawner;

impl LiquislimeUnit for Spawner {
    fn update(time_elapsed: TimeInterval) {
        let added_amount_per_second = SlimeAmount::from_integer(1000);
        let added_amount = added_amount_per_second * time_elapsed.to_seconds();
        get_own_position().add_slime_amount(added_amount);

        if let Some(position) = get_mouse_touch() {
            // TODO: needs proper fixing
            // println!("Hello from wasi {:?}", position);
            position
                .to_tile_position()
                .add_slime_amount(added_amount * 3);
        }
    }
}

export_liquislime_unit!(Spawner);
