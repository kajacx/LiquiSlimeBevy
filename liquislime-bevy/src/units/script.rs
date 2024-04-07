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
    pub settings_value: SettingsValue,
    pub settings_description: SettingsDescription,
}

impl ScriptInstance {
    pub fn new(instance: UnitInstance, settings: Option<SettingsValue>) -> Self {
        let settings_description = instance.settings_description();
        Self {
            instance: Arc::new(instance),
            settings_value: settings.unwrap_or_else(|| settings_description.default_value()),
            settings_description,
        }
    }

    pub fn init(&self) {
        self.instance.init(&self.settings_value);
    }

    pub fn update(&self, time_elapsed: TimeInterval) {
        self.instance.update(time_elapsed);
    }
}
