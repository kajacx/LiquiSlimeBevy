use super::*;

#[derive(Debug, Clone)]
pub struct Settings(pub String);

enum SettingsValue {
    String(String),
    Number(i32),
    SlimeAmount(SlimeAmount),
    Object(Vec<(String, SettingsDescription)>),
}

enum SettingsDescription {
    String(StringSetting),
    Number(NumberSetting<i32>),
    SlimeAmount(NumberSetting<SlimeAmount>),
    Object(Vec<(String, SettingsDescription)>),
    Option(Box<SettingsDescription>),
}

#[derive(Default, Debug)]
struct StringSetting {
    default_value: Option<String>,
    max_length: Option<u32>,
}

#[derive(Default, Debug)]
struct NumberSetting<T> {
    default_value: Option<T>,
    min_value: Option<T>,
    max_value: Option<T>,
    slider: Option<bool>,
}
