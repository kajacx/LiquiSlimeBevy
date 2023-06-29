// use liquislime_api::*;

// struct Spawner;

// impl LiquislimeUnit for Spawner {
//     fn update(time_elapsed: TimeInterval) {
//         let added_amount_per_second = SlimeAmount::from_integer(1000);

//         let amount = added_amount_per_second * time_elapsed.to_seconds();

//         add_slime_amount(get_own_position(), amount);
//     }
// }

// export_liquislime_unit_my!(Spawner);

wit_bindgen::generate!({
    path: "../../protocol.wit",
    world: "liquislime-unit",
});

struct Spawner;

impl LiquislimeUnit for Spawner {
    fn update(_: TimeInterval) {}
}

export_liquislime_unit!(Spawner);
