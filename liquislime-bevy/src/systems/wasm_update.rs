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
    use_world_reference_in(world, |token| {
        update_all_units(TimeInterval::from_milliseconds(20.0), token); // TODO: proper time elapsed
    });
}
