use super::{SettingsDescription, SettingsTempValue, SettingsUiDisplay, SettingsValue};
use crate::{
    api::{ApiSlimeAmount, DynValue},
    helpers::ResultLogger,
};
use anyhow::{Context, Result};
use bevy_egui::egui::Ui;
use ref_cast::RefCast;
use std::fmt::Write;

#[derive(Debug, Clone)]
pub struct SdObject(Vec<(String, SettingsDescription)>);

impl SdObject {
    pub fn deserialize(value: &DynValue) -> Result<Self> {
        let mut pairs = vec![];
        for (key, value) in value
            .field("values")?
            .fields()
            .context("values should be an object")?
        {
            pairs.push((key.to_string(), SettingsDescription::deserialize(&value)?));
        }
        Ok(Self(pairs))
    }

    pub fn default_value(&self) -> DynValue {
        DynValue::Object(
            self.0
                .iter()
                .map(|(key, value)| (key.to_string(), value.default_value()))
                .collect(),
        )
    }
}

impl SettingsUiDisplay for SdObject {
    fn display_ui_element(&self, ui: &mut Ui, value: &mut SettingsTempValue) {
        for (key, obj_value) in &self.0 {
            ui.label(key);
            obj_value.display_ui_element(
                ui,
                SettingsTempValue::ref_cast_mut(value.0.field_mut_anyway(key)),
            )
        }
    }

    fn save_settings(&self, temp_value: &SettingsTempValue, value: &mut SettingsValue) {
        for (key, obj_value) in &self.0 {
            obj_value.save_settings(
                SettingsTempValue::ref_cast(temp_value.0.field(&key).expect("TODO: user error")),
                SettingsValue::ref_cast_mut(value.0.field_mut(&key).expect("TODO: user error")),
            )
        }
    }

    fn reset_settings(&self, value: &SettingsValue, temp_value: &mut SettingsTempValue) {
        for (key, obj_value) in &self.0 {
            obj_value.reset_settings(
                SettingsValue::ref_cast(value.0.field(&key).expect("TODO: user error")),
                SettingsTempValue::ref_cast_mut(temp_value.0.field_mut_anyway(&key)),
            )
        }
    }
}
