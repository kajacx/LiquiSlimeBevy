use super::{SettingsTempValue, SettingsUiDisplay, SettingsValue};
use crate::api::DynValue;
use anyhow::Result;
use bevy_egui::egui::Ui;

#[derive(Debug, Clone)]
pub struct SdNone;

impl SdNone {
    pub fn deserialize(_value: &DynValue) -> Result<Self> {
        Ok(Self)
    }

    pub fn default_value(&self) -> DynValue {
        DynValue::Null
    }
}

impl SettingsUiDisplay for SdNone {
    fn display_ui_element(&self, _ui: &mut Ui, _value: &mut SettingsTempValue) {}

    fn save_settings(&self, _temp_value: &SettingsTempValue, _value: &mut SettingsValue) {}

    fn reset_settings(&self, _value: &SettingsValue, _temp_value: &mut SettingsTempValue) {}
}
