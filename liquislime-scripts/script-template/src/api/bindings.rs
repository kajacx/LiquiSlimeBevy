use super::{allocate_bytes_impl, free_bytes_impl, FatPtr, FromWasmAbi, ToWasmAbi};
use crate::{
    plugin::UserScript,
    settings::{SettingsDescription, SettingsValue},
    Faction, Instance, Position, ScriptTemplate, SettingsTemplate, SlimeAmount, TilePosition,
    TimeInterval, Unit,
};
use std::collections::HashMap;

#[no_mangle]
pub fn init() {
    crate::log("Entering init");
    *crate::INSTANCES.try_lock().unwrap() = Some(HashMap::new());
    crate::log("Exiting init");
}

#[no_mangle]
pub fn describe_settings() -> <SettingsDescription as ToWasmAbi>::Abi {
    crate::log("Entering describe_settings");
    let result =
        <<UserScript as ScriptTemplate>::Settings as SettingsTemplate>::describe_settings();
    crate::log("Converting describe_settings");
    let abi = result.to_wasm_abi();
    crate::log("Exiting describe_settings");
    abi
}

#[no_mangle]
pub fn default_settings() -> <SettingsValue as ToWasmAbi>::Abi {
    crate::log("Entering default_settings");
    let result = <<UserScript as ScriptTemplate>::Settings as SettingsTemplate>::default_value();
    crate::log("Converting default_settings");
    let abi = result.to_wasm_abi();
    crate::log("Exiting default_settings");
    abi
}

#[no_mangle]
pub fn new_instance(
    instance_id: <Instance as FromWasmAbi>::Abi,
    settings: <SettingsValue as FromWasmAbi>::Abi,
) {
    crate::log("Entering new_instance");

    let settings_value = SettingsValue::from_wasm_abi(settings);

    crate::log("Unwrapping settings value");

    let settings_value = settings_value.expect("TODO: user error");

    crate::log("Parsing settings value");

    let settings =
        <<UserScript as ScriptTemplate>::Settings as SettingsTemplate>::parse(settings_value);

    crate::log("Unwrapping parsed settings");

    let settings = settings.expect("TODO: User error");

    crate::log("Creating instance");

    let instance = UserScript::new_instance(settings);

    crate::log("Inserting instance to global storage");

    crate::INSTANCES
        .try_lock()
        .unwrap()
        .as_mut()
        .unwrap()
        .insert(Instance::from_wasm_abi(instance_id).unwrap(), instance);

    crate::log("Exiting new_instance");
}

#[no_mangle]
pub fn change_settings(
    instance_id: <Instance as FromWasmAbi>::Abi,
    settings: <SettingsValue as FromWasmAbi>::Abi,
) {
    crate::log("Entering change_settings");

    let settings_value = SettingsValue::from_wasm_abi(settings);

    crate::log("Unwrapping settings value");

    let settings_value = settings_value.expect("TODO: user error");

    crate::log("Parsing settings value");

    let settings =
        <<UserScript as ScriptTemplate>::Settings as SettingsTemplate>::parse(settings_value);

    crate::log("Unwrapping parsed settings");

    let settings = settings.expect("TODO: User error");

    crate::log("Calling user's change_settings");

    crate::INSTANCES
        .try_lock()
        .unwrap()
        .as_mut()
        .unwrap()
        .get_mut(&Instance::from_wasm_abi(instance_id).unwrap())
        .expect("TODO: user error, but it shouldn't really happen")
        .change_settings(settings);

    crate::log("Exiting change_settings");
}

#[no_mangle]
pub fn update(
    instance_id: <Instance as FromWasmAbi>::Abi,
    time_elapsed: <TimeInterval as FromWasmAbi>::Abi,
) {
    crate::INSTANCES
        .try_lock()
        .unwrap()
        .as_mut()
        .unwrap()
        .get_mut(&Instance::from_wasm_abi(instance_id).unwrap())
        .expect("TODO: user error, but it shouldn't really happen")
        .update(TimeInterval::from_wasm_abi(time_elapsed).unwrap());
}

#[link(wasm_import_module = "liquislime_api")]
extern "C" {
    pub fn level_width() -> <i32 as FromWasmAbi>::Abi;
    pub fn level_height() -> <i32 as FromWasmAbi>::Abi;

    pub fn get_current_unit() -> <Unit as FromWasmAbi>::Abi;
    pub fn get_current_instance() -> <Instance as FromWasmAbi>::Abi;

    pub fn get_own_faction() -> <Faction as FromWasmAbi>::Abi;
    pub fn get_own_position() -> <TilePosition as FromWasmAbi>::Abi;

    pub fn get_slime_amount(
        faction: <Faction as ToWasmAbi>::Abi,
        position: <TilePosition as ToWasmAbi>::Abi,
    ) -> <SlimeAmount as FromWasmAbi>::Abi;
    pub fn set_slime_amount(
        faction: <Faction as ToWasmAbi>::Abi,
        position: <TilePosition as ToWasmAbi>::Abi,
        amount: <SlimeAmount as ToWasmAbi>::Abi,
    );

    pub fn get_mouse_position() -> <Option<Position> as FromWasmAbi>::Abi;
    pub fn is_mouse_pressed() -> <bool as FromWasmAbi>::Abi;

    pub fn log(msg: <&str as ToWasmAbi>::Abi);
}

#[no_mangle]
pub fn allocate_bytes(len: u32) -> <FatPtr as ToWasmAbi>::Abi {
    allocate_bytes_impl(len).to_wasm_abi()
}

#[no_mangle]
pub fn free_bytes(ptr: <FatPtr as FromWasmAbi>::Abi) {
    free_bytes_impl(FatPtr::from_wasm_abi(ptr).unwrap())
}
