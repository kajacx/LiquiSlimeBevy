use bevy::prelude::*;

use crate::units::{
    api_spec::types::TimeInterval, global_storage::use_world_reference_in, update_all_units,
};

pub struct WasmUpdatePlugin;

impl Plugin for WasmUpdatePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::PreUpdate, update_wasm_plugins);
    }
}

fn update_wasm_plugins(world: &mut World) {
    let time_resource = world
        .get_resource::<Time>()
        .expect("Time resource should exist");

    let time_elapsed = TimeInterval::from_seconds(time_resource.delta_seconds_f64());

    use_world_reference_in(world, |token| {
        update_all_units(time_elapsed, token);
    });
}
