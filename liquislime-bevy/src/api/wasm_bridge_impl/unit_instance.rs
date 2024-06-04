use crate::{
    api::{SettingsDescription, SettingsValue},
    helpers::ResultLogger,
};

use bevy::{prelude::*, tasks::AsyncComputeTaskPool};
use serde::Deserialize;

pub struct UnitInstance {
    store: UnitStore,
    instance: bindgen::LiquislimeUnit,
}

impl UnitInstance {
    pub fn new(store: UnitStore, instance: bindgen::LiquislimeUnit) -> Self {
        Self { store, instance }
    }

    pub fn settings_description(&self) -> SettingsDescription {
        let settings_string = self
            .instance
            .call_describe_settings(&mut *self.store.store_mut())
            .log_err_or_else("Instance's describe settings threw an error", || {
                "{}".to_string()
            });

        serde_json::from_str(&settings_string).expect("TODO: user error")
    }

    pub fn init(&self, settings: &SettingsValue) {
        self.instance
            .call_init(&mut *self.store.store_mut(), &settings.0.to_string())
            .log_err("Instance's call init function caused an error.");
    }

    pub fn change_settings(&self, settings: &SettingsValue) {
        self.instance
            .call_change_settings(&mut *self.store.store_mut(), &settings.0.to_string())
            .log_err("Instance's change settings function caused an error"); // TODO: log "inner" error
    }

    pub fn update(&self, time_elapsed: crate::api::TimeInterval) {
        let time_elapsed = bindgen::TimeInterval {
            fragments: time_elapsed.0,
        };

        self.instance
            .call_update(&mut *self.store.store_mut(), time_elapsed)
            .log_err("Instance's call function caused an error.");
        // TODO: add unit ID or something
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
