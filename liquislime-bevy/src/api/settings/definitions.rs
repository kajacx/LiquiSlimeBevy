use super::{object, SdFloat64, SdNone, SdObject, SdString};
use crate::{api::DynValue, helpers::ResultLogger};
use anyhow::{bail, Context, Result};
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
    String(SdString),
    Float64(SdFloat64),
    Object(SdObject),
}

impl SettingsDescription {
    pub fn deserialize(value: &DynValue) -> Result<Self> {
        let tag = value
            .field("type")
            .context("Get setting description type")?
            .as_str()
            .context("Setting description type to string")?;

        Ok(if tag == "None" {
            Self::None(SdNone::deserialize(value)?)
        } else if tag == "Float64" {
            Self::Float64(SdFloat64::deserialize(value)?)
        } else if tag == "String" {
            Self::String(SdString::deserialize(value)?)
        } else if tag == "Object" {
            Self::Object(SdObject::deserialize(value)?)
        } else {
            bail!("Unknown SD tag: {tag}")
        })
    }

    pub fn default_value(&self) -> DynValue {
        match self {
            Self::None(none) => none.default_value(),
            Self::Float64(value) => value.default_value(),
            Self::String(text) => text.default_value(),
            Self::Object(values) => values.default_value(),
        }
    }
}

pub trait SettingsUiDisplay {
    fn display_ui_element(&self, ui: &mut Ui, value: &mut SettingsTempValue);

    fn save_settings(&self, temp_value: &SettingsTempValue, value: &mut SettingsValue);

    fn reset_settings(&self, value: &SettingsValue, temp_value: &mut SettingsTempValue);
}

impl SettingsUiDisplay for SettingsDescription {
    fn display_ui_element(&self, ui: &mut Ui, value: &mut SettingsTempValue) {
        match self {
            Self::None(none) => none.display_ui_element(ui, value),
            Self::Float64(number) => number.display_ui_element(ui, value),
            Self::String(text) => text.display_ui_element(ui, value),
            Self::Object(object) => object.display_ui_element(ui, value),
        }
    }

    fn save_settings(&self, temp_value: &SettingsTempValue, value: &mut SettingsValue) {
        match self {
            Self::None(none) => none.save_settings(temp_value, value),
            Self::Float64(number) => number.save_settings(temp_value, value),
            Self::String(text) => text.save_settings(temp_value, value),
            Self::Object(object) => object.save_settings(temp_value, value),
        }
    }

    fn reset_settings(&self, value: &SettingsValue, temp_value: &mut SettingsTempValue) {
        match self {
            Self::None(none) => none.reset_settings(value, temp_value),
            Self::Float64(number) => number.reset_settings(value, temp_value),
            Self::String(text) => text.reset_settings(value, temp_value),
            Self::Object(object) => object.reset_settings(value, temp_value),
        }
    }
}
