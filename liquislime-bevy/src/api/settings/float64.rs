use super::{SettingsTempValue, SettingsUiDisplay, SettingsValue};
use crate::{
    api::{ApiSlimeAmount, DynValue},
    helpers::ResultLogger,
};
use bevy_egui::egui::Ui;
use std::fmt::Write;

#[derive(Debug, Clone)]
pub struct SdFloat64;

impl SettingsUiDisplay for SdFloat64 {
    fn display_ui_element(&self, ui: &mut Ui, value: &mut SettingsTempValue) {
        ui.text_edit_singleline(value.0.as_string_mut_anyway("0"));
    }

    fn save_settings(&self, temp_value: &SettingsTempValue, value: &mut SettingsValue) {
        let temp_text = temp_value.0.as_str().log_err_or(
            "SdSlimeAmount temp value is not text, this shouldn't happen",
            "",
        );

        if let Ok(amount) = temp_text.parse::<f64>() {
            value.0 = DynValue::Float64(amount);
        } else {
            // TODO: report error
        }
    }

    fn reset_settings(&self, value: &SettingsValue, temp_value: &mut SettingsTempValue) {
        let amount = value.0.as_f64().log_err_or(
            "SdSlimeAmount value is not SlimeAmount, this shouldn't happen",
            ApiSlimeAmount::default(),
        );

        let temp_text = temp_value.0.as_string_mut_anyway("");
        temp_text.clear();
        write!(temp_text, "{}", amount);
    }

    fn deserialize(_value: &DynValue) -> Self {
        Self
    }
}
