use super::{LoadedScript, Script};
use crate::api::{ApiTimeInterval, ScriptImpl, SettingsTempValue, SettingsValue};
use anyhow::bail;
use slab::Slab;
use try_lock::TryLock;
use wasm_bridge::Result;

pub(super) static INSTANCES: TryLock<Slab<()>> = TryLock::new(Slab::new());

#[derive(Debug)]
pub struct ScriptInstance {
    id: u32,
    script: LoadedScript,
    settings: SettingsValue,
    pub temp_settings: SettingsTempValue,
    instance_state: TryLock<InstanceState>,
}

#[derive(Debug)]
enum InstanceState {
    Initialized,
    NotInitialized,
}

impl ScriptInstance {
    pub fn new(script: LoadedScript, id: u32, settings: SettingsValue) -> Self {
        Self {
            id,
            script,
            settings,
            temp_settings: SettingsTempValue(rmpv::Value::Nil), // TODO:
            instance_state: TryLock::new(InstanceState::NotInitialized),
        }
    }

    pub fn change_settings(&mut self, settings: SettingsValue) -> Result<()> {
        self.script
            .script_impl()
            .change_settings(self.id, &settings)?;
        self.settings = settings;
        Ok(())
    }

    pub fn update(&self, time_elapsed: ApiTimeInterval) -> Result<()> {
        self.script.script_impl().update(self.id, time_elapsed)
    }
}
