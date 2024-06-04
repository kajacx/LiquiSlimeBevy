use super::{
    memory_manage::FatPtr, FromWasmAbi, FromWasmAbiSimple, ScriptImpl, StoreData, ToWasmAbi,
    ToWasmAbiSimple, WasmAccess,
};
use crate::api::{ApiInstance, ApiTimeInterval, SettingsDescription, SettingsValue};
use std::fmt::Debug;
use try_lock::TryLock;
use wasm_bridge::{Result, Store, StoreContextMut, TypedFunc};

pub struct Exports {
    init_func: TypedFunc<(), ()>,
    describe_settings_func: TypedFunc<(), <SettingsDescription as FromWasmAbi>::Abi>,
    new_instance_func: TypedFunc<
        (
            <ApiInstance as ToWasmAbi>::Abi,
            <SettingsValue as ToWasmAbi>::Abi,
        ),
        (),
    >,
    change_settings_func: TypedFunc<
        (
            <ApiInstance as ToWasmAbi>::Abi,
            <SettingsValue as ToWasmAbi>::Abi,
        ),
        (),
    >,
    update_func: TypedFunc<
        (
            <ApiInstance as ToWasmAbi>::Abi,
            <ApiTimeInterval as ToWasmAbi>::Abi,
        ),
        (),
    >,
    allocate_bytes_func: TypedFunc<u32, <FatPtr as FromWasmAbi>::Abi>,
    free_bytes_func: TypedFunc<<FatPtr as ToWasmAbi>::Abi, ()>,
}

impl Exports {
    pub fn new(
        mut context: &mut Store<StoreData>,
        instance: &wasm_bridge::Instance,
    ) -> Result<Self> {
        Ok(Self {
            init_func: instance.get_typed_func(&mut context, "init")?,
            describe_settings_func: instance.get_typed_func(&mut context, "describe_settings")?,
            new_instance_func: instance.get_typed_func(&mut context, "new_instance")?,
            change_settings_func: instance.get_typed_func(&mut context, "change_settings")?,
            update_func: instance.get_typed_func(&mut context, "update")?,
            allocate_bytes_func: instance.get_typed_func(&mut context, "allocate_bytes")?,
            free_bytes_func: instance.get_typed_func(&mut context, "free_bytes")?,
        })
    }

    pub fn init(&self, context: &mut WasmAccess) -> Result<()> {
        self.init_func.call(&mut context.store, ())
    }

    pub fn describe_settings(&self, mut context: &mut WasmAccess) -> Result<SettingsDescription> {
        let settings_abi = self.describe_settings_func.call(&mut context.store, ())?;

        SettingsDescription::from_wasm_abi(context, settings_abi)
    }

    pub fn new_instance(
        &self,
        context: &mut WasmAccess,
        id: u32,
        settings: &SettingsValue,
    ) -> Result<()> {
        let settings_abi = settings.to_wasm_abi(context)?;

        println!(
            "SETTINGS ABI: {:?} {:?} {:?}",
            settings,
            settings_abi,
            FatPtr::from_wasm_abi_simple(settings_abi)
        );

        self.new_instance_func
            .call(&mut context.store, (id, settings_abi))
    }

    pub fn change_settings(
        &self,
        context: &mut WasmAccess,
        id: u32,
        settings: &SettingsValue,
    ) -> Result<()> {
        let settings_abi = settings.to_wasm_abi(context)?;

        self.change_settings_func
            .call(&mut context.store, (id, settings_abi))
    }

    pub fn update(
        &self,
        context: &mut WasmAccess,
        id: u32,
        time_elapsed: ApiTimeInterval,
    ) -> Result<()> {
        self.update_func
            .call(&mut context.store, (id, time_elapsed.to_wasm_abi_simple()))
    }

    pub fn allocate_bytes(&self, mut context: &mut WasmAccess, len: u32) -> Result<FatPtr> {
        self.allocate_bytes_func
            .call(&mut context.store, len)
            .map(FatPtr::from_wasm_abi_simple)
    }

    pub fn free_bytes(&self, context: &mut WasmAccess, bytes_ptr: FatPtr) -> Result<()> {
        self.free_bytes_func
            .call(&mut context.store, bytes_ptr.to_wasm_abi_simple())
    }
}

impl Debug for Exports {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Exports cannot derive Debug.")
    }
}
