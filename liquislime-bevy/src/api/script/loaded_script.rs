use super::{LiquislimeImports, Script, ScriptInstance};
use crate::{
    api::{ScriptImpl, SettingsDescription, SettingsValue},
    assets::ScriptAsset,
};
use anyhow::Result;
use bevy::{prelude::*, render::settings};
use std::sync::Arc;
use try_lock::TryLock;

#[derive(Debug, Clone)]
pub struct LoadedScript(Arc<LoadedScriptInner>); // TODO: asset now can use this

#[derive(Debug)]
struct LoadedScriptInner {
    settings_description: SettingsDescription,
    default_settings: SettingsValue,
    script_impl: ScriptImpl,
}

impl LoadedScript {
    pub fn from_bytes(bytes: &[u8], imports: impl LiquislimeImports) -> Result<Self> {
        let script_impl = ScriptImpl::from_bytes(bytes, imports)?;
        let settings_description = script_impl.describe_settings()?;
        let default_settings = SettingsValue(settings_description.default_value());

        Ok(Self(Arc::new(LoadedScriptInner {
            settings_description,
            default_settings,
            script_impl,
        })))
    }

    pub fn new_instance(&self, script: Script, settings: SettingsValue) -> Result<ScriptInstance> {
        let instance = ScriptInstance::new(script, self.clone(), settings);
        // TODO: remove instance on failure
        instance.with_settings(|settings| {
            self.0
                .script_impl
                .new_instance(instance.id(), &settings.current_settings)
        })?;
        Ok(instance)
    }

    pub fn settings_description(&self) -> &SettingsDescription {
        &self.0.settings_description
    }

    pub fn default_settings(&self) -> &SettingsValue {
        &self.0.default_settings
    }

    pub fn script_impl(&self) -> &ScriptImpl {
        &self.0.script_impl
    }
}
