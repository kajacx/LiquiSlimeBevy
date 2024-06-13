use crate::api::ToWasmAbi;
use anyhow::Result;
use plugin::UserScript;
use settings::{SettingsDescription, SettingsValue};
use std::{collections::HashMap, fmt::Display};
use try_lock::TryLock;

#[allow(unused)]
mod types;
#[allow(unused)]
use types::*;

mod plugin;

mod settings;

mod api;

static INSTANCES: TryLock<Option<HashMap<Instance, UserScript>>> = TryLock::new(None);

pub trait SettingsTemplate: Sized {
    fn describe_settings() -> SettingsDescription;

    fn default_value() -> SettingsValue;

    fn parse(value: SettingsValue) -> Result<Self>;
}

pub trait ScriptTemplate {
    type Settings: SettingsTemplate;

    fn new_instance(settings: Self::Settings) -> Self;

    fn change_settings(&mut self, settings: Self::Settings);

    fn update(&mut self, time_elapsed: TimeInterval);
}

#[allow(unused)]
fn log(msg: impl Display) {
    unsafe { api::log((&*format!("{msg}")).to_wasm_abi()) }
}
