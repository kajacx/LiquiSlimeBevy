use crate::api::LoadedScript;
use bevy::prelude::*;
use try_lock::TryLock;

#[derive(Debug, Asset, TypePath)]
pub struct ScriptAsset {
    pub bytes: Vec<u8>,
}
