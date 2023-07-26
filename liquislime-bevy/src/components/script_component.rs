use std::sync::{Arc, Mutex};

use bevy::{prelude::*, tasks::AsyncComputeTaskPool};

use crate::{assets::ScriptModule, units::ScriptInstance};

#[derive(Debug, Component)]
pub struct ScriptComponent {
    inner: Arc<Mutex<ScriptComponentInner>>,
}

#[derive(Debug)]
enum ScriptComponentInner {
    Loaded(ScriptInstance),
    AssetLoading(Handle<ScriptModule>),
    // Instantiating,
}

impl ScriptComponent {
    pub fn new(handle: Handle<ScriptModule>) -> Self {
        Self {
            inner: Arc::new(Mutex::new(ScriptComponentInner::AssetLoading(handle))),
        }
    }

    pub fn try_load(&mut self, script_assets: &Assets<ScriptModule>) {
        use ScriptComponentInner::*;

        let mut lock = self.inner.lock().unwrap();

        let new = match &*lock {
            AssetLoading(handle) => {
                // TODO: use .map
                let module = script_assets.get(&handle);
                if let Some(module) = module {
                    Some(self.spawn_instantiate_task(module))
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

    fn spawn_instantiate_task(&self, module: &ScriptModule) -> ScriptComponentInner {
        // use ScriptComponentInner::*;
        // let thread_pool = AsyncComputeTaskPool::get();

        // let self_clone = self.clone();
        // let future = module.instantiate();

        // thread_pool.spawn_local(async move {
        //     let instance = future.await;
        //     *self_clone.inner.lock().unwrap() = Loaded(instance)
        // });
        // Instantiating

        let instance = module.instantiate();
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
}
