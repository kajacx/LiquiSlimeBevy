use std::{fmt::Debug, sync::Arc};

use bevy::prelude::{Assets, Handle};

use crate::{api::*, assets::ScriptModule};

#[derive(Debug)]
pub struct ScriptStore {
    store: UnitStore,
}

#[derive(Debug, Clone)]
pub struct ScriptInstance {
    instance: Arc<UnitInstance>,
    settings_value: serde_json::Value,
    settings_description: SettingsDescription,
}

impl ScriptInstance {
    pub fn new(instance: UnitInstance) -> Self {
        Self {
            settings_value: instance.default_settings(),
            settings_description: instance.settings_description(),
            instance: Arc::new(instance),
        }
    }

    pub fn init(&self, settings: &Settings) {
        self.instance.init(settings);
    }

    pub fn update(&self, time_elapsed: TimeInterval) {
        self.instance.update(time_elapsed);
    }
}
