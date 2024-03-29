use std::sync::{Arc, Mutex};

use bevy::{prelude::*, tasks::AsyncComputeTaskPool};
use wasm_bridge::Engine;

use crate::{
    api::{Settings, UnitModule},
    assets::ScriptModule,
    units::ScriptInstance,
};

#[derive(Clone, Debug, Component)]
pub struct ScriptsComponent(pub Vec<(ScriptComponent, Settings)>);

#[derive(Clone, Debug)]
pub struct ScriptComponent {
    inner: Arc<Mutex<ScriptComponentInner>>,
}

#[derive(Debug)]
enum ScriptComponentInner {
    Loaded(ScriptInstance),
    AssetLoading(Handle<ScriptModule>),
    OnlineCompiling, // Compiling source from text field
}

impl ScriptComponent {
    pub fn new(handle: Handle<ScriptModule>) -> Self {
        Self {
            inner: Arc::new(Mutex::new(ScriptComponentInner::AssetLoading(handle))),
        }
    }

    pub fn try_load(&mut self, script_assets: &Assets<ScriptModule>, settings: &Settings) {
        use ScriptComponentInner::*;

        let mut lock = self.inner.lock().unwrap();

        let new = match &*lock {
            AssetLoading(handle) => {
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

    fn spawn_instantiate_task(
        &self,
        module: &ScriptModule,
        settings: &Settings,
    ) -> ScriptComponentInner {
        let instance = module.instantiate();
        instance.init(settings);
        ScriptComponentInner::Loaded(instance)
    }

    pub fn instance(&self) -> Option<ScriptInstance> {
        let mut lock = self.inner.lock().unwrap();
        use ScriptComponentInner::*;

        match &mut *lock {
            Loaded(instance) => Some(instance.clone()),
            _ => None,
        }
    }

    pub fn load_from_bytes(&self, bytes: Vec<u8>, settings: &Settings) {
        let thread_pool = AsyncComputeTaskPool::get();

        *self.inner.lock().unwrap() = ScriptComponentInner::OnlineCompiling;

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
