use crate::api::Script;
use bevy::prelude::*;

#[derive(Clone, Debug, Component)]
pub struct ScriptComponent(pub Script);

// TODO: Safety?
unsafe impl Send for ScriptComponent {}
unsafe impl Sync for ScriptComponent {}
