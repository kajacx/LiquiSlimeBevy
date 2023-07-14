use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use bevy::prelude::*;

use crate::{
    api::TimeInterval,
    assets::ScriptModule,
    units::{
        global_storage::{get_world, set_current_unit, WorldRefToken},
        MaybeLoadedScript, UnitId,
    },
};

#[derive(Clone, Debug, Resource)]
pub struct UnitScriptMap(HashMap<UnitId, Arc<Mutex<MaybeLoadedScript>>>);

impl UnitScriptMap {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn register_new_unit(&mut self, id: UnitId, script: MaybeLoadedScript) {
        self.0.insert(id, Arc::new(Mutex::new(script)));
    }

    // TODO: this function is kind of horrible ...
    pub fn update_all_units(&self, time_elapsed: TimeInterval, _world_ref: &WorldRefToken) {
        for (unit_id, maybe_script) in self.0.iter() {
            let mut world = get_world();
            let mut asset = world.resource_mut::<Assets<ScriptModule>>();

            let mut script = maybe_script
                .try_lock()
                .expect("Should lock maybe loaded script");

            let script = script.try_get_script(&mut *asset);

            drop(world);

            // info!("UPDATE? {:?}", script);

            if let Some(script) = script {
                set_current_unit(*unit_id);
                script.update(time_elapsed);
            }
        }
    }
}
