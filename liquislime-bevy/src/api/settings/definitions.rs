use super::{SdFloat64, SdNone, SdText};
use crate::{api::DynValue, helpers::ResultLogger};
use bevy_egui::egui::Ui;
use ref_cast::RefCast;
use std::collections::HashMap;

#[derive(ref_cast::RefCast)]
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct SettingsValue(pub DynValue);

#[derive(ref_cast::RefCast)]
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct SettingsTempValue(pub DynValue);

#[derive(Debug, Clone)]
pub enum SettingsDescription {
    None(SdNone),
    Text(SdText),
    Float64(SdFloat64),
}

pub trait SettingsUiDisplay {
    fn display_ui_element(&self, ui: &mut Ui, value: &mut SettingsTempValue);

    fn save_settings(&self, temp_value: &SettingsTempValue, value: &mut SettingsValue);

    fn reset_settings(&self, value: &SettingsValue, temp_value: &mut SettingsTempValue);

    fn deserialize(value: &DynValue) -> Self;
}

impl SettingsUiDisplay for SettingsDescription {
    fn display_ui_element(&self, ui: &mut Ui, value: &mut SettingsTempValue) {
        match self {
            Self::None(none) => none.display_ui_element(ui, value),
            Self::Float64(number) => number.display_ui_element(ui, value),
            Self::Text(text) => text.display_ui_element(ui, value),
        }
    }

    fn save_settings(&self, temp_value: &SettingsTempValue, value: &mut SettingsValue) {
        match self {
            Self::None(none) => none.save_settings(temp_value, value),
            Self::Float64(number) => number.save_settings(temp_value, value),
            Self::Text(text) => text.save_settings(temp_value, value),
        }
    }

    fn reset_settings(&self, value: &SettingsValue, temp_value: &mut SettingsTempValue) {
        match self {
            Self::None(none) => none.reset_settings(value, temp_value),
            Self::Float64(number) => number.reset_settings(value, temp_value),
            Self::Text(text) => text.reset_settings(value, temp_value),
        }
    }

    fn deserialize(value: &DynValue) -> Self {
        let text = value.as_str().expect("TODO: rework");
        if text == "None" {
            Self::None(SdNone)
        } else if text == "f64" {
            Self::Float64(SdFloat64)
        } else if text == "Text" {
            Self::Text(SdText)
        } else {
            panic!("Unknown SD tag: {text}")
        }
    }
}
