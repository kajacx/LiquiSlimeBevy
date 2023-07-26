use super::*;
use bevy::{prelude::*, tasks::AsyncComputeTaskPool};

pub struct UnitInstance {
    store: UnitStore,
    instance: bindgen::LiquislimeUnit,
}

impl UnitInstance {
    pub fn new(store: UnitStore, instance: bindgen::LiquislimeUnit) -> Self {
        Self { store, instance }
    }

    pub fn update(&self, time_elapsed: crate::api::TimeInterval) {
        let time_elapsed = bindgen::TimeInterval {
            fragments: time_elapsed.0,
        };

        let future = self
            .instance
            .call_update(&mut *self.store.store_mut(), time_elapsed)
            .expect("TODO: user error");
    }
}

// SAFETY: Bevy says it runs on only one "thread" (web worker) on the web
#[cfg(target_arch = "wasm32")]
unsafe impl Send for UnitInstance {}
#[cfg(target_arch = "wasm32")]
unsafe impl Sync for UnitInstance {}

impl std::fmt::Debug for UnitInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UnitInstance cannot derive Debug")
    }
}
