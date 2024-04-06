use std::collections::HashMap;

use super::*;

#[derive(Debug, Clone)]
pub struct Settings(pub String);

#[derive(Debug, Clone, serde::Deserialize)]
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
