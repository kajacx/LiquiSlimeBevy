use crate::api::ToWasmAbi;
use api::SettingsValue;
use plugin::ScriptInstance;
use std::{collections::HashMap, fmt::Display};
use try_lock::TryLock;

#[allow(unused)]
mod types;
#[allow(unused)]
use types::*;

mod plugin;

pub mod api;

static INSTANCES: TryLock<Option<HashMap<Instance, ScriptInstance>>> = TryLock::new(None);

trait LiquislimeScript {
    type Settings: TryFrom<SettingsValue>;

    // TODO: this should be removed and obtained from the Settings type instead
    fn describe_settings() -> rmpv::Value;

    fn new_instance(settings: Self::Settings) -> Self;

    fn change_settings(&mut self, settings: Self::Settings);

    fn update(&mut self, time_elapsed: TimeInterval);
}

#[allow(unused)]
fn log(msg: impl Display) {
    unsafe { api::log((&*format!("{msg}")).to_wasm_abi()) }
}
