use std::collections::HashMap;

use crate::units::{api_spec::types::TimeInterval, global_storage::set_current_unit};

type UnitMap = HashMap<UnitId, Script>;

static UNIT_SCRIPT_MAP: Mutex<UnitMap> = Mutex::new(HashMap::new());

pub fn register_new_unit(id: UnitId, script: Script) {
    (&mut *get_map()).insert(id, script);
}

pub fn update_all_units(time_elapsed: TimeInterval) {
    for (unit_id, script) in get_map().pairs() {
        set_current_unit(unaait_id);
        script.update(time_interval);
    }
}

fn get_map() -> impl DerefMut<Target = UnitMap> {
    UNIT_SCRIPT_MAP.lock().expect("Should lock unit script map")
}
