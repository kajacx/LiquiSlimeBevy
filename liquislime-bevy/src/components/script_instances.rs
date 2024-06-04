use crate::api::ScriptInstance;
use bevy::prelude::*;
use std::sync::Arc;

#[derive(Debug, Component)]
pub struct ScriptInstances(pub Vec<Arc<ScriptInstance>>);

// TODO: Safety
unsafe impl Send for ScriptInstances {}
unsafe impl Sync for ScriptInstances {}
