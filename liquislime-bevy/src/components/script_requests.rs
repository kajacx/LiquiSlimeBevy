use crate::api::ScriptRequest;
use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct ScriptRequests(pub Vec<ScriptRequest>);
