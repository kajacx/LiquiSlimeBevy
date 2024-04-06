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
    settings_value: SettingsValue,
    settings_description: SettingsDescription,
}

impl ScriptInstance {
    pub fn new(instance: UnitInstance) -> Self {
        let settings_description = instance.settings_description();
        Self {
            instance: Arc::new(instance),
            settings_value: settings_description.default_value(),
            settings_description,
        }
    }

    pub fn init(&self, settings: &SettingsValue) {
        self.instance.init(settings);
    }

    pub fn update(&self, time_elapsed: TimeInterval) {
        self.instance.update(time_elapsed);
    }
}
