use bevy::prelude::*;

use crate::units::{api_spec::types::TimeInterval, global_storage::set_world, update_all_units};

pub struct WasmUpdatePlugin;

impl Plugin for WasmUpdatePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::PreUpdate, update_wasm_plugins);
    }
}

fn update_wasm_plugins(world: &mut World) {
    // TODO: add a compile-time check that these are called correctly?
    set_world(world);

    update_all_units(TimeInterval::from_milliseconds(20.0)); // TODO: proper time elapsed
}
