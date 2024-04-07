use std::collections::HashMap;

use crate::{helpers::ResultLogger, systems::gui::SettingsUiDisplay};

use super::*;

#[derive(Debug, Clone)]
pub struct SettingsValue(pub serde_json::Value);

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum SettingsDescription {
    String(StringSetting),
    Number(NumberSetting<i32>),
    SlimeAmount(NumberSetting<SlimeAmount>),
    Object(Vec<(String, SettingsDescription)>),
    Option(Box<SettingsDescription>),
    // Object(HashMap<String, SettingsDescription>),
}

#[derive(Default, Debug, Clone, serde::Deserialize)]
pub struct StringSetting {
    pub default_value: Option<String>,
    pub max_length: Option<u32>,
}

#[derive(Default, Debug, Clone, serde::Deserialize)]
pub struct NumberSetting<T> {
    pub default_value: Option<T>,
    pub min_value: Option<T>,
    pub max_value: Option<T>,
    pub slider: Option<bool>,
}

impl SettingsDescription {
    pub fn default_value(&self) -> SettingsValue {
        SettingsValue(match self {
            Self::Number(number) => serde_json::json!(number.default_value.unwrap_or_default()),
            Self::String(string) => {
                serde_json::json!(string.default_value.as_deref().unwrap_or_default())
            }
            Self::SlimeAmount(amount) => {
                serde_json::json!(amount.default_value.unwrap_or_default())
            }
            // Self::Option(None) => serde_json::json!(null), TODO: default null value
            Self::Option(inner) => inner.default_value().0,
            Self::Object(fields) => {
                let mut json = serde_json::json!({});
                for (name, value) in fields {
                    json[name] = value.default_value().0;
                }
                json
            }
        })
    }
}

impl SettingsUiDisplay for StringSetting {
    fn display_ui_element(&self, ui: &mut bevy_egui::egui::Ui, value: SettingsValue) {
        ui.label(
            value
                .0
                .as_str()
                .log_err_or("settings value is not string", ""),
        );
    }
}

impl SettingsUiDisplay for NumberSetting<SlimeAmount> {
    fn display_ui_element(&self, ui: &mut bevy_egui::egui::Ui, value: SettingsValue) {
        ui.label(
            serde_json::from_value::<SlimeAmount>(value.0)
                .log_err_or("settings value is not slime amount", SlimeAmount::new())
                .as_float()
                .to_string(),
        );
    }
}

impl SettingsUiDisplay for SettingsDescription {
    fn display_ui_element(&self, ui: &mut bevy_egui::egui::Ui, value: SettingsValue) {
        match self {
            Self::SlimeAmount(amount) => amount.display_ui_element(ui, value),
            Self::String(string) => string.display_ui_element(ui, value),
            Self::Object(object) => {
                for (name, val) in object {
                    ui.label(name);
                    // TODO: clone
                    val.display_ui_element(ui, SettingsValue(value.0[name].clone()));
                }
            }
            _ => todo!(),
        }
    }
}
