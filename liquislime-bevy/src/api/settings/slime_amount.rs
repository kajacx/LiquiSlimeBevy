use super::{SettingsTempValue, SettingsUiDisplay, SettingsValue};
use crate::{api::ApiSlimeAmount, helpers::ResultLogger};
use bevy_egui::egui::Ui;
use std::fmt::Write;

#[derive(Debug, Clone)]
pub struct SdSlimeAmount;

impl SettingsUiDisplay for SdSlimeAmount {
    fn display_ui_element(&self, ui: &mut Ui, value: &mut SettingsTempValue) {
        ui.text_edit_singleline(value.0.as_string_mut_anyway("0"));
    }

    fn save_settings(&self, temp_value: &SettingsTempValue, value: &mut SettingsValue) {
        let temp_text = temp_value.0.as_str().log_err_or(
            "SdSlimeAmount temp value is not text, this shouldn't happen",
            "",
        );

        if let Ok(amount) = temp_text.parse::<f64>() {
            value.0 = ApiSlimeAmount::from_float(amount).into()
        } else {
            // TODO: report error
        }
    }

    fn reset_settings(&self, value: &SettingsValue, temp_value: &mut SettingsTempValue) {
        let amount = value.0.as_slime_amount().log_err_or(
            "SdSlimeAmount value is not SlimeAmount, this shouldn't happen",
            ApiSlimeAmount::default(),
        );

        let temp_text = temp_value.0.as_string_mut_anyway("");
        temp_text.clear();
        write!(temp_text, "{}", amount.as_float());
    }

    fn deserialize(_value: &crate::api::DynValue) -> Self {
        Self
    }
}
