use super::{LiquislimeImports, Script, ScriptInstance, INSTANCES};
use crate::{
    api::{ScriptImpl, SettingsDescription, SettingsValue},
    assets::ScriptAsset,
};
use bevy::{prelude::*, render::settings};
use std::sync::Arc;
use try_lock::TryLock;
use wasm_bridge::Result;

#[derive(Debug, Clone)]
pub struct LoadedScript(Arc<LoadedScriptInner>); // TODO: asset now can use this

#[derive(Debug)]
struct LoadedScriptInner {
    settings_description: SettingsDescription,
    script_impl: ScriptImpl,
}

impl LoadedScript {
    pub fn from_bytes(bytes: &[u8], imports: impl LiquislimeImports) -> Result<Self> {
        let script_impl = ScriptImpl::from_bytes(bytes, imports)?;
        let settings_description = script_impl.describe_settings()?;

        Ok(Self(Arc::new(LoadedScriptInner {
            settings_description,
            script_impl,
        })))
    }

    pub fn new_instance(&self, settings: SettingsValue) -> Result<ScriptInstance> {
        let id = INSTANCES.try_lock().unwrap().insert(()) as u32;
        self.new_instance_with_id(id, settings)
    }

    pub fn new_instance_with_id(&self, id: u32, settings: SettingsValue) -> Result<ScriptInstance> {
        self.0.script_impl.new_instance(id, &settings)?;
        Ok(ScriptInstance::new(self.clone(), id, settings))
    }

    pub fn settings_description(&self) -> &SettingsDescription {
        &self.0.settings_description
    }

    pub fn script_impl(&self) -> &ScriptImpl {
        &self.0.script_impl
    }
}
