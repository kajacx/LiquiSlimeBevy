use crate::api::{ApiTimeInterval, ScriptStatus};
use crate::assets::ScriptAsset;
use crate::components::{
    ScriptComponent, ScriptInstances, ScriptRequests, TilePositionComponent, UnitId,
};
use crate::units::global_storage::set_current_unit;
use crate::{helpers::Phase, units::global_storage::use_world_reference_in};
use bevy::prelude::*;
use std::sync::Arc;

pub struct WasmUpdatePlugin;

impl Plugin for WasmUpdatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_wasm_scripts.in_set(Phase::WasmUpdate));
    }
}

fn update_wasm_scripts(world: &mut World) {
    let time_resource = world.resource::<Time>();

    let time_elapsed = ApiTimeInterval::from_seconds(time_resource.delta_seconds_f64());

    // "Limit" update delta, but it will "lag" below 30 FPS
    let time_elapsed = time_elapsed.min(ApiTimeInterval::from_seconds(1.0 / 30.0));

    // "Ready check" to see if all scripts are loaded
    let mut all_ready = true;
    for script in world.query::<&ScriptComponent>().iter(world) {
        // TODO: load asset server lazily
        let asset_server = world.resource::<Assets<ScriptAsset>>();
        if script.0.try_load(asset_server) == ScriptStatus::NotLoaded {
            println!("Script {} not ready", script.0.name());
            all_ready = false;
        }
    }

    // Try to initialize scripts
    for (mut instances, mut requests) in world
        .query::<(&mut ScriptInstances, &mut ScriptRequests)>()
        .iter_mut(world)
    {
        requests.0.retain(|request| {
            let resolved = request.try_initialize().expect("TODO: user error");
            let keep = if let Some(instance) = resolved {
                instances.0.push(instance);
                false
            } else {
                all_ready = false;
                true
            };
            keep
        });
    }

    if all_ready {
        let mut units_and_ids = vec![];

        for (id, instances) in world.query::<(&UnitId, &ScriptInstances)>().iter(world) {
            for instance in &instances.0 {
                units_and_ids.push((*id, *instance));
            }
        }

        use_world_reference_in(world, move || {
            for (unit_id, script) in units_and_ids {
                set_current_unit(unit_id);
                script.update(time_elapsed).expect("TODO: user error");
            }
        });
    } else {
        // TODO: display a loading bar or something?
        info!("Loading waiting for asset load ...");
    }
}
