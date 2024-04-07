use std::{fmt::Debug, sync::Arc};

use bevy::prelude::{Assets, Handle};
use serde_json::json;

use crate::{api::*, assets::ScriptModule, systems::gui::SettingsUiDisplay};

#[derive(Debug)]
pub struct ScriptStore {
    store: UnitStore,
}

#[derive(Debug, Clone)]
pub struct ScriptInstance {
    instance: Arc<UnitInstance>,
    pub settings_description: SettingsDescription,
    pub settings_value: SettingsValue,
    pub settings_temp_value: SettingsTempValue,
}

impl ScriptInstance {
    pub fn new(instance: UnitInstance, settings: Option<SettingsValue>) -> Self {
        let settings_description = instance.settings_description();
        let settings_value = settings.unwrap_or_else(|| settings_description.default_value());
        let mut settings_temp_value = SettingsTempValue(json!(null));
        settings_description.reset_settings(&settings_value, &mut settings_temp_value);

        Self {
            instance: Arc::new(instance),
            settings_description,
            settings_value,
            settings_temp_value,
        }
    }

    pub fn init(&self) {
        self.instance.init(&self.settings_value);
    }

    pub fn change_settings(&self) {
        self.instance.change_settings(&self.settings_value);
    }

    pub fn update(&self, time_elapsed: TimeInterval) {
        self.instance.update(time_elapsed);
    }
}
