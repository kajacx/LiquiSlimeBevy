use super::{LoadedScript, Script};
use crate::api::{ApiTimeInterval, ScriptImpl, SettingsTempValue, SettingsValue};
use anyhow::bail;
use slab::Slab;
use try_lock::TryLock;
use wasm_bridge::Result;

static INSTANCES: TryLock<Slab<()>> = TryLock::new(Slab::new());

#[derive(Debug)]
pub struct ScriptInstance {
    id: u32,
    script: Script,
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
    pub fn new(script: Script, settings: SettingsValue) -> Self {
        let id = INSTANCES.try_lock().unwrap().insert(()) as u32;

        Self {
            id,
            script,
            settings,
            temp_settings: SettingsTempValue(rmpv::Value::Nil), // TODO:
            instance_state: TryLock::new(InstanceState::NotInitialized),
        }
    }

    pub fn try_initialize(&self) -> Result<()> {
        let mut lock = self.instance_state.try_lock().unwrap();
        if matches!(*lock, InstanceState::NotInitialized) {
            self.script
                .with_loaded(|loaded| {
                    loaded
                        .new_instance(self.id, &self.settings)
                        .expect("TODO: user error");
                    *lock = InstanceState::Initialized;
                })
                .or(Ok(()))
        } else {
            Ok(())
        }
    }

    pub fn change_settings(&mut self, settings: SettingsValue) -> Result<()> {
        self.with_initialized(|loaded_script| {
            loaded_script
                .script_impl()
                .change_settings(self.id, &settings)
        })?;
        self.settings = settings;
        Ok(())
    }

    pub fn update(&self, time_elapsed: ApiTimeInterval) -> Result<()> {
        self.with_initialized(|loaded_script| {
            loaded_script.script_impl().update(self.id, time_elapsed)
        })
    }

    fn with_initialized(&self, callback: impl FnOnce(&LoadedScript) -> Result<()>) -> Result<()> {
        if let InstanceState::Initialized = *self.instance_state.try_lock().unwrap() {
            self.script.with_loaded(move |loaded| callback(loaded))?
        } else {
            bail!("Instance state is not initialized.");
        }
    }
}
