use std::sync::{Arc, Mutex};

use bevy::{prelude::*, tasks::AsyncComputeTaskPool};
use wasm_bridge::Engine;

use crate::{
    api::{SettingsDescription, SettingsTempValue, SettingsValue, UnitModule},
    assets::ScriptModule,
    units::ScriptInstance,
};

#[derive(Clone, Debug, Component)]
pub struct ScriptsComponent(pub Vec<ScriptHolder>);

#[derive(Clone, Debug)]
pub struct ScriptHolder {
    pub name: &'static str, // TODO: this is kind of hacky, refactor
    inner: Arc<Mutex<ScriptInner>>,
}

#[derive(Debug)]
enum ScriptInner {
    Loaded(ScriptInstance),
    AssetLoading(Handle<ScriptModule>, SettingsValue),
    OnlineCompiling, // Compiling source from text field
}

impl ScriptHolder {
    pub fn new(name: &'static str, handle: Handle<ScriptModule>, settings: SettingsValue) -> Self {
        Self {
            name,
            inner: Arc::new(Mutex::new(ScriptInner::AssetLoading(handle, settings))),
        }
    }

    pub fn try_load(&mut self, script_assets: &Assets<ScriptModule>) {
        use ScriptInner::*;

        let mut lock = self.inner.lock().unwrap();

        let new = match &*lock {
            AssetLoading(handle, settings) => {
                let module = script_assets.get(*&handle);
                if let Some(module) = module {
                    let instance = module.instantiate(Some(settings.clone()));
                    instance.init();
                    Some(ScriptInner::Loaded(instance))
                } else {
                    None
                }
            }
            _ => None,
        };

        if let Some(new) = new {
            *lock = new;
        }
    }

    pub fn with_settings(
        &self,
        callback: impl FnOnce(
            Option<(
                &SettingsDescription,
                &mut SettingsValue,
                &mut SettingsTempValue,
            )>,
        ),
    ) {
        let mut lock = self.inner.try_lock().unwrap();
        callback(lock.settings());
    }

    pub fn instance(&self) -> Option<ScriptInstance> {
        let mut lock = self.inner.lock().unwrap();
        use ScriptInner::*;

        match &mut *lock {
            Loaded(instance) => Some(instance.clone()),
            _ => None,
        }
    }

    pub fn change_settings(&self) {
        let mut lock = self.inner.lock().unwrap();
        use ScriptInner::*;

        match &mut *lock {
            Loaded(instance) => instance.change_settings(),
            _ => println!("TODO: print a warning"),
        }
    }

    pub fn load_from_bytes(&self, bytes: Vec<u8>, settings: &SettingsValue) {
        let thread_pool = AsyncComputeTaskPool::get();

        *self.inner.lock().unwrap() = ScriptInner::OnlineCompiling;

        let self_clone = self.clone();
        let settings = settings.clone();

        thread_pool.spawn_local(async move {
            let unit_module = UnitModule::from_bytes(&bytes).await;
            let script_module = ScriptModule::new("custom-unit".into(), unit_module);

            // FIXME: repair this
            // *self_clone.inner.lock().unwrap() =
            //     self_clone.spawn_instantiate_task(&script_module, &settings);
        });
    }
}

impl ScriptInner {
    pub fn settings(
        &mut self,
    ) -> Option<(
        &SettingsDescription,
        &mut SettingsValue,
        &mut SettingsTempValue,
    )> {
        match self {
            Self::Loaded(instance) => Some((
                &instance.settings_description,
                &mut instance.settings_value,
                &mut instance.settings_temp_value,
            )),
            _ => None,
        }
    }
}
