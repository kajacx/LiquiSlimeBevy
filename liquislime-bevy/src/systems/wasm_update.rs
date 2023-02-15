use std::collections::HashMap;

type UnitMap = HashMap<UnitId, Script>;

static UNIT_SCRIPT_MAP: Mutex<UnitMap> = Mutex::new(HashMap::new());

pub fn register_new_unit(id: UnitId, script: Script) {
    (&mut *get_map()).insert(id, script);
}

fn get_map() -> impl DerefMut<Target = UnitMap> {
    UNIT_SCRIPT_MAP.lock().expect("Should lock unit script map")
}
