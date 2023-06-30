use bevy::prelude::*;

use crate::api::TimeInterval;
use crate::{
    helpers::WasmUpdate, resources::UnitScriptMap, units::global_storage::use_world_reference_in,
};

pub struct WasmUpdatePlugin;

impl Plugin for WasmUpdatePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_wasm_plugins.in_base_set(WasmUpdate::Update));
    }
}

fn update_wasm_plugins(world: &mut World) {
    let time_resource = world
        .get_resource::<Time>()
        .expect("Time resource should exist");

    // TODO: limit update time to 30 FPS?
    let time_elapsed = TimeInterval::from_seconds(time_resource.delta_seconds_f64());

    let units_resource = world
        .get_resource::<UnitScriptMap>()
        .expect("Unit script map resource should exist");

    // TODO: unfortunate clone, but otherwise, the unit map is borrowed from the world
    //let unit_keys = units_resource.keys().to_owned();
    let unit_map = units_resource.clone();

    use_world_reference_in(world, |token| {
        unit_map.update_all_units(time_elapsed, token);
    });
}
