use crate::api::ScriptInstance;
use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct ScriptInstances(pub Vec<ScriptInstance>);

// TODO: Safety
unsafe impl Send for ScriptInstances {}
unsafe impl Sync for ScriptInstances {}
