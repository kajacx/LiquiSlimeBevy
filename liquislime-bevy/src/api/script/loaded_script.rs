use super::{Script, ScriptInstance};
use crate::{
    api::{ScriptImpl, SettingsDescription, SettingsValue},
    assets::ScriptAsset,
};
use bevy::{prelude::*, render::settings};
use std::sync::Arc;
use try_lock::TryLock;
use wasm_bridge::Result;

#[derive(Debug)]
pub struct LoadedScript(Arc<LoadedScriptInner>); // TODO: asset now can use this

#[derive(Debug)]
struct LoadedScriptInner {
    settings_description: SettingsDescription,
    script_impl: ScriptImpl,
}

impl LoadedScript {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let script_impl = ScriptImpl::from_bytes(bytes)?;
        let settings_description = script_impl.describe_settings()?;

        Ok(Self(Arc::new(LoadedScriptInner {
            settings_description,
            script_impl,
        })))
    }

    pub fn new_instance(&self, id: u32, settings: &SettingsValue) -> Result<()> {
        self.0.script_impl.new_instance(id, settings)
    }

    pub fn settings_description(&self) -> &SettingsDescription {
        &self.0.settings_description
    }

    pub fn script_impl(&self) -> &ScriptImpl {
        &self.0.script_impl
    }
}
