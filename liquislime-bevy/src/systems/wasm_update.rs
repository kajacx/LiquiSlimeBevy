use bevy::prelude::*;

use crate::api::TimeInterval;
use crate::components::{ScriptComponent, TilePositionComponent};
use crate::units::global_storage::set_current_unit;
use crate::units::UnitId;
use crate::{helpers::Phase, units::global_storage::use_world_reference_in};

pub struct WasmUpdatePlugin;

impl Plugin for WasmUpdatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_wasm_plugins.in_set(Phase::WasmUpdate));
    }
}

fn update_wasm_plugins(world: &mut World) {
    let time_resource = world
        .get_resource::<Time>()
        .expect("Time resource should exist");

    // TODO: limit update time to 30 FPS?
    let time_elapsed = TimeInterval::from_seconds(time_resource.delta_seconds_f64());

    // TODO: unfortunate clone, but otherwise, the unit map is borrowed from the world
    //let unit_keys = units_resource.keys().to_owned();
    // let unit_map = units_resource.clone();

    let mut units_and_ids = vec![];
    let mut all_ready = true;

    let mut units = world.query::<(&ScriptComponent, &UnitId)>();
    for (unit, id) in units.iter(world) {
        // println!("HELLO {:?}", unit);
        let instance = unit.instance();
        if let Some(instance) = unit.instance() {
            units_and_ids.push((id.clone(), instance.clone()));
        } else {
            all_ready = false;
            break;
        }
    }

    if all_ready {
        use_world_reference_in(world, move |_| {
            for (unit_id, script) in units_and_ids {
                set_current_unit(unit_id);
                script.update(time_elapsed);
            }
        });
    } else {
        // TODO: display a loading bar or something?
        info!("Loading waiting for asset load ...");
    }

    // use_world_reference_in(world, |token| {
    //     unit_map.update_all_units(time_elapsed, token);
    // });
}
