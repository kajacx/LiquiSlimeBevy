use std::collections::HashMap;

use ref_cast::RefCast;
use serde_json::json;

use crate::{helpers::ResultLogger, systems::gui::SettingsUiDisplay};

use super::*;

#[derive(ref_cast::RefCast)]
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct SettingsValue(pub serde_json::Value);

#[derive(ref_cast::RefCast)]
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct SettingsTempValue(pub serde_json::Value);

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
    fn display_ui_element(&self, ui: &mut bevy_egui::egui::Ui, value: &mut SettingsTempValue) {
        match &mut value.0 {
            serde_json::Value::String(string) => ui.text_edit_singleline(string),
            // TODO: bad settings
            _ => ui.label("bad settings"),
        };
    }

    fn save_settings(&self, tmp_value: &SettingsTempValue, value: &mut SettingsValue) {
        value.0 = tmp_value.0.clone();
    }

    fn reset_settings(&self, value: &SettingsValue, tmp_value: &mut SettingsTempValue) {
        tmp_value.0 = value.0.clone();
    }
}

impl SettingsUiDisplay for NumberSetting<SlimeAmount> {
    fn display_ui_element(&self, ui: &mut bevy_egui::egui::Ui, value: &mut SettingsTempValue) {
        match &mut value.0 {
            serde_json::Value::String(string) => ui.text_edit_singleline(string),
            // TODO: bad settings
            _ => ui.label("bad settings"),
        };
    }

    fn save_settings(&self, tmp_value: &SettingsTempValue, value: &mut SettingsValue) {
        value.0 = json!(SlimeAmount::from_float(
            tmp_value
                .0
                .as_str()
                .expect("TODO: user error")
                .parse()
                .log_err_or("TODO: better error display", 0.0)
        ));
    }

    fn reset_settings(&self, value: &SettingsValue, tmp_value: &mut SettingsTempValue) {
        tmp_value.0 = json!(format!(
            "{}",
            serde_json::from_value::<SlimeAmount>(value.0.clone())
                .expect("TODO: user error")
                .as_float()
        ));
    }
}

impl SettingsUiDisplay for SettingsDescription {
    fn display_ui_element(&self, ui: &mut bevy_egui::egui::Ui, value: &mut SettingsTempValue) {
        match self {
            Self::SlimeAmount(amount) => amount.display_ui_element(ui, value),
            Self::String(string) => string.display_ui_element(ui, value),
            Self::Object(object) => {
                for (name, val) in object {
                    ui.label(name);
                    val.display_ui_element(
                        ui,
                        SettingsTempValue::ref_cast_mut(
                            value
                                .0
                                .as_object_mut()
                                .expect("TODO: user error")
                                .get_mut(name)
                                .expect("TODO: user error"),
                        ),
                    );
                }
            }
            _ => todo!(),
        }
    }

    fn save_settings(&self, tmp_value: &SettingsTempValue, value: &mut SettingsValue) {
        match self {
            Self::SlimeAmount(amount) => amount.save_settings(tmp_value, value),
            Self::String(string) => string.save_settings(tmp_value, value),
            Self::Object(object) => {
                value.0 = json!({});
                for (name, val) in object {
                    value.0[name] = json!(null);
                    val.save_settings(
                        SettingsTempValue::ref_cast(
                            tmp_value.0.get(name).expect("TODO: User error"),
                        ),
                        SettingsValue::ref_cast_mut(value.0.get_mut(name).unwrap()),
                    )
                }
            }
            _ => todo!(),
        }
    }

    fn reset_settings(&self, value: &SettingsValue, tmp_value: &mut SettingsTempValue) {
        match self {
            Self::SlimeAmount(amount) => amount.reset_settings(value, tmp_value),
            Self::String(string) => string.reset_settings(value, tmp_value),
            Self::Object(object) => {
                tmp_value.0 = json!({});
                for (name, val) in object {
                    tmp_value.0[name] = json!(null);
                    val.reset_settings(
                        SettingsValue::ref_cast(value.0.get(name).expect("TODO: user error")),
                        SettingsTempValue::ref_cast_mut(tmp_value.0.get_mut(name).unwrap()),
                    )
                }
            }
            _ => todo!(),
        }
    }
}
