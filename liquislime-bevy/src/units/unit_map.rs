use std::{collections::HashMap, ops::DerefMut, sync::Mutex};

use bevy::render::once_cell::sync::Lazy;

use crate::units::{api_spec::types::TimeInterval, global_storage::set_current_unit};

use super::{global_storage::WorldRefToken, script::Script, UnitId};

type UnitMap = HashMap<UnitId, Script>;

static UNIT_SCRIPT_MAP: Lazy<Mutex<UnitMap>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub fn register_new_unit(id: UnitId, script: Script) {
    (&mut *get_map()).insert(id, script);
}

pub fn update_all_units(time_elapsed: TimeInterval, _world_ref: &WorldRefToken) {
    for (unit_id, script) in get_map().iter() {
        set_current_unit(unit_id.clone()); // TODO: how to copy a copy type?
        script.update(time_elapsed);
    }
}

fn get_map() -> impl DerefMut<Target = UnitMap> {
    UNIT_SCRIPT_MAP.lock().expect("Should lock unit script map")
}
