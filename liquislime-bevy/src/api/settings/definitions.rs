use super::{SdNone, SdSlimeAmount, SdText};
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
    SlimeAmount(SdSlimeAmount),
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
            Self::Text(text) => text.display_ui_element(ui, value),
            Self::SlimeAmount(amount) => amount.display_ui_element(ui, value),
        }
    }

    fn save_settings(&self, temp_value: &SettingsTempValue, value: &mut SettingsValue) {
        match self {
            Self::None(none) => none.save_settings(temp_value, value),
            Self::Text(text) => text.save_settings(temp_value, value),
            Self::SlimeAmount(amount) => amount.save_settings(temp_value, value),
        }
    }

    fn reset_settings(&self, value: &SettingsValue, temp_value: &mut SettingsTempValue) {
        match self {
            Self::None(none) => none.reset_settings(value, temp_value),
            Self::Text(text) => text.reset_settings(value, temp_value),
            Self::SlimeAmount(amount) => amount.reset_settings(value, temp_value),
        }
    }

    fn deserialize(value: &DynValue) -> Self {
        let text = value.as_str().expect("TODO: rework");
        if text == "None" {
            Self::None(SdNone)
        } else if text == "Text" {
            Self::Text(SdText)
        } else if text == "SlimeAmount" {
            Self::SlimeAmount(SdSlimeAmount)
        } else {
            panic!("Unknown SD tag: {text}")
        }
    }
}
