use crate::api::ToWasmAbi;
use std::{collections::HashMap, fmt::Display};
use try_lock::TryLock;
use user_script::UserScript;

#[allow(unused)]
mod types;
#[allow(unused)]
use types::*;

mod user_script;

mod settings;
#[allow(unused)]
use settings::*;

mod api;

static INSTANCES: TryLock<Option<HashMap<Instance, UserScript>>> = TryLock::new(None);

pub trait ScriptTemplate {
    type Settings: From<DynValue>;

    fn describe_settings() -> SettingsDescription;

    fn new_instance(settings: Self::Settings) -> Self;

    fn change_settings(&mut self, settings: Self::Settings);

    fn update(&mut self, time_elapsed: TimeInterval);
}

#[allow(unused)]
fn log(msg: impl Display) {
    unsafe { api::log((&*format!("{msg}")).to_wasm_abi()) }
}

fn hook_impl(info: &std::panic::PanicInfo) {
    log("!!!!! SCRIPT PANIC START !!!!!");
    log(format!("{info}"));
    log("!!!!! SCRIPT PANIC  END  !!!!!");
}
