use std::sync::{Arc, Mutex};

use bevy::{prelude::*, tasks::AsyncComputeTaskPool};
use wasm_bridge::Engine;

use crate::{
    api::{SettingsValue, UnitModule},
    assets::ScriptModule,
    units::ScriptInstance,
};

#[derive(Clone, Debug, Component)]
pub struct ScriptsComponent(pub Vec<ScriptHolder>);

#[derive(Clone, Debug)]
pub struct ScriptHolder {
    inner: Arc<Mutex<ScriptInner>>,
}

#[derive(Debug)]
enum ScriptInner {
    Loaded(ScriptInstance),
    AssetLoading(Handle<ScriptModule>, SettingsValue),
    OnlineCompiling, // Compiling source from text field
}

impl ScriptHolder {
    pub fn new(handle: Handle<ScriptModule>, settings: SettingsValue) -> Self {
        Self {
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
                    Some(self.spawn_instantiate_task(module, settings))
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

    pub fn get_settings(&self) -> SettingsValue {
        // TODO: ugly clone, refactor
        self.inner.try_lock().unwrap().get_settings().clone()
    }

    fn spawn_instantiate_task(
        &self,
        module: &ScriptModule,
        settings: &SettingsValue,
    ) -> ScriptInner {
        let instance = module.instantiate();
        instance.init(settings);
        ScriptInner::Loaded(instance)
    }

    pub fn instance(&self) -> Option<ScriptInstance> {
        let mut lock = self.inner.lock().unwrap();
        use ScriptInner::*;

        match &mut *lock {
            Loaded(instance) => Some(instance.clone()),
            _ => None,
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

            *self_clone.inner.lock().unwrap() =
                self_clone.spawn_instantiate_task(&script_module, &settings);
        });
    }
}

impl ScriptInner {
    pub fn get_settings(&self) -> &SettingsValue {
        match self {
            Self::AssetLoading(_, settings) => settings,
            Self::Loaded(instance) => &instance.settings_value,
            Self::OnlineCompiling => todo!("Get settings when online compiling"),
        }
    }
}
