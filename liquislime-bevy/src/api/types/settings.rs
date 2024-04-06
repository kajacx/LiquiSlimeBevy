use std::collections::HashMap;

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
