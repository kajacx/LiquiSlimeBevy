use std::collections::HashMap;

use bevy::prelude::*;

use crate::units::{api_spec::types::TimeInterval, global_storage::WorldRefToken, Script, UnitId};

#[derive(Clone, Debug, Resource)]
pub struct UnitScriptMap(HashMap<UnitId, Script>);

impl UnitScriptMap {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn register_new_unit(&mut self, id: UnitId, script: Script) {
        self.0.insert(id, script);
    }

    pub fn update_all_units(&self, time_elapsed: TimeInterval, _world_ref: &WorldRefToken) {
        for (unit_id, script) in self.0.iter() {
            set_current_unit(*unit_id);
            script.update(time_elapsed);
        }
    }
}
