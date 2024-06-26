use super::{create_linker, get_engine, get_store, Exports, FromWasmAbi, StoreData, WasmAccess};
use crate::api::{
    script, ApiTimeInterval, LiquislimeImports, LoadedScript, SettingsDescription, SettingsValue,
};
use anyhow::Result;
use atomic_refcell::AtomicRefCell;
use bevy::ecs::schedule::Dag;
use slab::Slab;
use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
};
use wasmi::{AsContext, AsContextMut, Memory, Module, Store, StoreContextMut};

static SCRIPTS: AtomicRefCell<Slab<ScriptData>> = AtomicRefCell::new(Slab::new());

#[derive(Debug, Clone, Copy)]
pub struct ScriptImpl(usize);

#[derive(Debug)]
pub struct ScriptData {
    exports: Exports,
    memory: Memory,
}

// TODO: memory safety
unsafe impl Send for ScriptData {}
unsafe impl Sync for ScriptData {}

impl ScriptImpl {
    pub(super) fn new() -> Self {
        Self(usize::MAX)
    }

    pub fn from_bytes(bytes: &[u8], imports: impl LiquislimeImports) -> Result<Self> {
        let data = ScriptData::from_bytes(bytes, imports)?;

        let script = Self(SCRIPTS.try_borrow_mut().unwrap().insert(data));

        script.with_store("ScriptImpl::from_bytes", |context, script| {
            script.exports.init(context)
        })?;

        Ok(script)
    }

    pub fn describe_settings(&self) -> Result<SettingsDescription> {
        self.with_store("ScriptImpl::describe_settings", |context, script| {
            script.exports.describe_settings(context)
        })
    }

    pub fn new_instance(&self, id: u32, settings: &SettingsValue) -> Result<()> {
        self.with_store("ScriptImpl::new_instance", |context, script| {
            script.exports.new_instance(context, id, settings)
        })
    }

    pub fn change_settings(&self, id: u32, settings: &SettingsValue) -> Result<()> {
        self.with_store("ScriptImpl::change_settings", |context, script| {
            script.exports.change_settings(context, id, settings)
        })
    }

    pub fn update(&self, id: u32, time_elapsed: ApiTimeInterval) -> Result<()> {
        self.with_store("ScriptImpl::update", |context, script| {
            script.exports.update(context, id, time_elapsed)
        })
    }

    fn with_store<T>(
        &self,
        name: &'static str,
        callback: impl FnOnce(&mut WasmAccess, &ScriptData) -> T,
    ) -> T {
        let mut store = get_store(name);
        self.set_current_script(store.as_context_mut());
        let mut wasm_context = WasmAccess {
            store: store.as_context_mut(),
        };
        let script = &SCRIPTS.try_borrow().unwrap()[self.0];
        callback(&mut wasm_context, script)
    }

    fn set_current_script(&self, mut context: StoreContextMut<StoreData>) {
        context.data_mut().current_script = *self;
    }

    pub(super) fn with_exports<T>(&self, callback: impl FnOnce(&Exports) -> T) -> T {
        callback(&SCRIPTS.try_borrow().unwrap()[self.0].exports)
    }

    pub(super) fn with_memory<T>(&self, callback: impl FnOnce(&Memory) -> T) -> T {
        callback(&SCRIPTS.try_borrow().unwrap()[self.0].memory)
    }

    pub(super) fn read_zero_terminated_string(
        &self,
        context: impl AsContext<Data = StoreData>,
        mut address: usize,
    ) -> Result<String> {
        self.with_memory(move |memory| {
            let mut bytes = vec![];
            let mut byte = [0u8, 0u8];
            loop {
                memory
                    .read(&context, address, &mut byte)
                    .expect("TODO: read byte for ZTS");
                println!("READ: {} {}", byte[0], byte[1]);
                if byte[0] == 0 && byte[1] == 0 {
                    break;
                } else {
                    bytes.push(u16::from_le_bytes(byte));
                    address = address + 2;
                }
            }
            Ok(String::from_utf16_lossy(&bytes))
        })
    }
}

impl ScriptData {
    fn from_bytes(bytes: &[u8], imports: impl LiquislimeImports) -> Result<Self> {
        let module = Module::new(get_engine(), bytes)?;

        let mut store = get_store("ScriptData::from_bytes");

        let linker = create_linker(imports)?;

        let instance_pre = linker.instantiate(&mut *store, &module)?;

        let instance = instance_pre.start(&mut *store)?;

        let exports = Exports::new(&mut store, &instance)?;

        let memory = instance.get_memory(&mut *store, "memory").unwrap();

        Ok(Self { exports, memory })
    }
}
