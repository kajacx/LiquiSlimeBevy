use std::sync::Mutex;
use std::{fmt::Debug, sync::Arc};

use bevy::prelude::{Assets, Handle};
use wasmtime::component::*;
use wasmtime::Config;
use wasmtime::Engine;
use wasmtime::Store;

use crate::helpers::ScriptAsset;

use crate::wit::*;

pub struct Script {
    // store: Arc<Mutex<wasmtime::Store>>,
    store: Mutex<wasmtime::Store<LiquislimeHost>>,
    instance: crate::wit::LiquislimeUnit,
}

// FIXME: Is it really safe to just implement these? Probably not ...
// Oh no. Oh no.
//#[cfg(target_arch = "wasm32")]
// unsafe impl Send for Script {}
//#[cfg(target_arch = "wasm32")]
// unsafe impl Sync for Script {}

impl Script {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut config = Config::new();
        config.wasm_component_model(true);

        let engine = Engine::new(&config).unwrap();
        let mut store = Store::new(&engine, LiquislimeHost);

        let component = Component::new(&store.engine(), &bytes).expect("create component");

        let mut linker = Linker::new(store.engine());

        LiquislimeUnit::add_to_linker(&mut linker, |state| state).unwrap();
        let (liquislime_unit, _instance) =
            LiquislimeUnit::instantiate(&mut store, &component, &linker).unwrap();

        Self {
            store: Mutex::new(store),
            instance: liquislime_unit,
        }
    }

    pub fn update(&self, time_elapsed: TimeInterval) {
        self.instance
            .call_update(&mut *self.store.try_lock().unwrap(), time_elapsed)
            .expect("TODO: update should log on error");
    }
}

impl Debug for Script {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Script cannot derive Debug, blame Wasmer")
    }
}

#[derive(Debug)]
pub enum MaybeLoadedScript {
    Loaded(Arc<Script>),
    Loading(Handle<ScriptAsset>), // TODO: add from impl
}

impl MaybeLoadedScript {
    pub fn new(handle: Handle<ScriptAsset>) -> Self {
        Self::Loading(handle)
    }

    pub fn try_get_script<'a>(
        &'a mut self,
        script_assets: &mut Assets<ScriptAsset>,
    ) -> Option<&'a Script> {
        self.try_load(script_assets);

        match self {
            Self::Loaded(script) => Some(script),
            Self::Loading(_) => None,
        }
    }

    fn try_load(&mut self, script_assets: &mut Assets<ScriptAsset>) {
        let loaded_script = if let Self::Loading(handle) = self {
            script_assets.get(handle)
        } else {
            None
        };

        if let Some(script) = loaded_script {
            *self = Self::Loaded(script.0.clone());
        }
    }
}
