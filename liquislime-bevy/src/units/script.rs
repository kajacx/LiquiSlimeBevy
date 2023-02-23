use std::fmt::Debug;

use super::api_spec::{bindings::Runtime, types::TimeInterval};

#[derive(Clone)]
pub struct Script {
    runtime: Runtime,
}

// FIXME: Is it really safe to just implement these? Probably not ...
#[cfg(target_arch = "wasm32")]
unsafe impl Send for Script {}
#[cfg(target_arch = "wasm32")]
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
