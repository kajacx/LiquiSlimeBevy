use super::{
    allocate_bytes_impl, free_bytes_impl, FatPtr, FromWasmAbi, SettingsDescription, SettingsValue,
    ToWasmAbi,
};
use crate::{
    Faction, Instance, LiquislimeScript, Position, SlimeAmount, TilePosition, TimeInterval, Unit,
};
use std::collections::HashMap;

#[no_mangle]
pub fn init() {
    crate::log("Entering init");
    // console_error_panic_hook::set_once(); // TODO: set up panic hook on the web
    *crate::INSTANCES.try_lock().unwrap() = Some(HashMap::new());
    crate::log("Exiting init");
}

#[no_mangle]
pub fn describe_settings() -> <SettingsDescription as FromWasmAbi>::Abi {
    crate::log("Entering describe_settings");
    let res = crate::ScriptInstance::describe_settings().to_wasm_abi();
    crate::log("Exiting describe_settings");
    res
}

#[no_mangle]
pub fn new_instance(
    instance_id: <Instance as FromWasmAbi>::Abi,
    settings: <SettingsValue as FromWasmAbi>::Abi,
) {
    crate::log("Entering new_instance");

    let settings_value = SettingsValue::from_wasm_abi(settings);

    crate::log("Got settings value");

    let settings = settings_value.try_into().expect("TODO: user error");

    crate::log("Parsed settings");

    let instance = crate::ScriptInstance::new_instance(settings);

    crate::INSTANCES
        .try_lock()
        .unwrap()
        .as_mut()
        .unwrap()
        .insert(Instance::from_wasm_abi(instance_id), instance);

    crate::log("Exiting new_instance");
}

#[no_mangle]
pub fn change_settings(
    instance_id: <Instance as FromWasmAbi>::Abi,
    settings: <SettingsValue as FromWasmAbi>::Abi,
) {
    crate::log("Entering change_settings");

    crate::INSTANCES
        .try_lock()
        .unwrap()
        .as_mut()
        .unwrap()
        .get_mut(&Instance::from_wasm_abi(instance_id))
        .expect("TODO: user error, but it shouldn't really happen")
        .change_settings(
            SettingsValue::from_wasm_abi(settings)
                .try_into()
                .expect("TODO: user error"),
        );

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
        .get_mut(&Instance::from_wasm_abi(instance_id))
        .expect("TODO: user error, but it shouldn't really happen")
        .update(TimeInterval::from_wasm_abi(time_elapsed));
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
    free_bytes_impl(FatPtr::from_wasm_abi(ptr))
}
