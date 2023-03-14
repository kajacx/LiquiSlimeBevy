use std::fmt::Debug;

use bevy::prelude::{Assets, Handle, ResMut};

use crate::helpers::RawBytes;

use super::api_spec::{bindings::Runtime, types::TimeInterval};

pub struct Script {
    runtime: Runtime,
}

// FIXME: Is it really safe to just implement these? Probably not ...
// Oh no. Oh no.
//#[cfg(target_arch = "wasm32")]
unsafe impl Send for Script {}
//#[cfg(target_arch = "wasm32")]
unsafe impl Sync for Script {}

impl Script {
    pub fn from_plugin_path(path: &str) -> Self {
        // TODO: what to share if multiple units have the same script?
        // TODO: propage error to caller
        let bytes = std::fs::read(path).expect("Should read path when creating script");
        Self::from_bytes(&bytes)
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let runtime = Runtime::new(bytes).expect("Should create script runtime from bytes");
        Self::new(runtime)
    }

    pub fn new(runtime: Runtime) -> Self {
        Self { runtime }
    }

    pub fn update(&self, time_elapsed: TimeInterval) {
        self.runtime
            .update(time_elapsed)
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
    Loaded(Script),
    Loading(Handle<RawBytes>), // TODO: add from impl
}

impl MaybeLoadedScript {
    pub fn new(handle: Handle<RawBytes>) -> Self {
        Self::Loading(handle)
    }

    pub fn try_get_script<'a>(
        &'a mut self,
        byte_assets: &mut Assets<RawBytes>,
    ) -> Option<&'a Script> {
        self.try_load(byte_assets);

        match self {
            Self::Loaded(script) => Some(script),
            Self::Loading(handle) => None,
        }
    }

    fn try_load(&mut self, byte_assets: &mut Assets<RawBytes>) {
        let loaded_script = if let Self::Loading(handle) = self {
            byte_assets.get(handle)
        } else {
            None
        };

        if let Some(script) = loaded_script {
            *self = Self::Loaded(Script::from_bytes(script.0.as_ref()));
        }
    }
}
