use std::{
    collections::HashMap,
    ops::DerefMut,
    sync::{Arc, Mutex},
};

use bevy::render::once_cell::sync::Lazy;

use crate::units::{api_spec::types::TimeInterval, global_storage::set_current_unit};

use super::{global_storage::WorldRefToken, script::Script, UnitId};

type UnitMap = HashMap<UnitId, Script>;

pub fn register_new_unit(id: UnitId, script: Script) {
    todo!()
}

pub fn update_all_units(time_elapsed: TimeInterval, _world_ref: &WorldRefToken) {
    todo!()
}
