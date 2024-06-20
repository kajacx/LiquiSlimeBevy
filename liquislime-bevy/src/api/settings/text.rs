use super::{SettingsTempValue, SettingsUiDisplay, SettingsValue};
use crate::{api::DynValue, helpers::ResultLogger};
use anyhow::{Context, Result};
use bevy_egui::egui::Ui;

#[derive(Debug, Clone)]
pub struct SdString {
    default_value: String,
}

impl SdString {
    pub fn deserialize(value: &DynValue) -> Result<Self> {
        Ok(Self {
            default_value: value
                .field("default_value")?
                .as_str()
                .context("default value on SdText is not of type string")?
                .to_string(),
        })
    }

    pub fn default_value(&self) -> DynValue {
        DynValue::String(self.default_value.clone())
    }
}

impl SettingsUiDisplay for SdString {
    fn display_ui_element(&self, ui: &mut Ui, value: &mut SettingsTempValue) {
        ui.text_edit_singleline(value.0.as_string_mut_anyway(""));
    }

    fn save_settings(&self, temp_value: &SettingsTempValue, value: &mut SettingsValue) {
        let temp_text = temp_value
            .0
            .as_str()
            .log_err_or("SdText temp value is not text, this shouldn't happen", "");

        let text = value.0.as_string_mut_anyway("");
        text.clear();
        text.push_str(temp_text);
    }

    fn reset_settings(&self, value: &SettingsValue, temp_value: &mut SettingsTempValue) {
        let text = value
            .0
            .as_str()
            .log_err_or("SdText value is not text, this shouldn't happen", "");

        let temp_text = temp_value.0.as_string_mut_anyway("");
        temp_text.clear();
        temp_text.push_str(text);
    }
}
