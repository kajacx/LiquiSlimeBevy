use super::{allocate_bytes_impl, free_bytes_impl, FatPtr, FromWasmAbi, ToWasmAbi};
use crate::{
    settings::SettingsDescription, user_script::UserScript, DynValue, Faction, Instance, Position,
    ScriptTemplate, TilePosition, TimeInterval, Unit,
};
use std::collections::HashMap;

#[no_mangle]
pub fn init() {
    crate::log("Entering init");
    std::panic::set_hook(Box::new(crate::hook_impl));
    crate::log("Panic hook set");
    *crate::INSTANCES.try_lock().unwrap() = Some(HashMap::new());
    crate::log("Exiting init");
}

#[no_mangle]
pub fn describe_settings() -> <SettingsDescription as ToWasmAbi>::Abi {
    crate::log("Entering describe_settings");
    let result = <UserScript as ScriptTemplate>::describe_settings();
    crate::log("Converting describe_settings");
    let abi = result.to_wasm_abi();
    crate::log("Exiting describe_settings");
    abi
}

#[no_mangle]
pub fn new_instance(
    instance_id: <Instance as FromWasmAbi>::Abi,
    settings: <DynValue as FromWasmAbi>::Abi,
) {
    crate::log("Entering new_instance");

    let settings = get_settings(settings);

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
    settings: <DynValue as FromWasmAbi>::Abi,
) {
    crate::log("Entering change_settings");

    let settings = get_settings(settings);

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

fn get_settings(
    settings: <DynValue as FromWasmAbi>::Abi,
) -> <UserScript as ScriptTemplate>::Settings {
    super::debug_print_fat_ptr(FatPtr::from_wasm_abi(settings).unwrap());

    let dyn_value = DynValue::from_wasm_abi(settings);

    crate::log("Unwrapping settings dyn value");

    let dyn_value = dyn_value.expect("TODO: user error");

    crate::log(format!("Parsing settings value: {dyn_value:?}"));

    dyn_value.into()
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
    ) -> f64;
    pub fn set_slime_amount(
        faction: <Faction as ToWasmAbi>::Abi,
        position: <TilePosition as ToWasmAbi>::Abi,
        amount: f64,
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
