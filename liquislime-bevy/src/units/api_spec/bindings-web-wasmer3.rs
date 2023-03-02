use super::types::*;
use fp_bindgen_support::{
    common::{abi::WasmAbi, mem::FatPtr},
    host::{
        errors::{InvocationError, RuntimeError},
        mem::{
            deserialize_from_slice, export_to_guest, export_to_guest_raw, import_from_guest,
            import_from_guest_raw, serialize_to_vec,
        },
        r#async::{create_future_value, future::ModuleRawFuture, resolve_async_value},
        runtime::RuntimeInstanceData,
    },
};
use std::cell::RefCell;
use wasmer::{
    imports, AsStoreMut, Function, FunctionEnv, FunctionEnvMut, Imports, Instance, Module, Store,
};

pub struct Runtime {
    store: RefCell<Store>,
    module: Module,
}

impl Runtime {
    pub fn new(wasm_module: impl AsRef<[u8]>) -> Result<Self, RuntimeError> {
        let store = Self::default_store();
        let module = Module::new(&store, wasm_module)?;
        Ok(Self {
            store: RefCell::new(store),
            module,
        })
    }

    fn default_store() -> wasmer::Store {
        Store::new(wasmer::Engine::default())
    }
}
