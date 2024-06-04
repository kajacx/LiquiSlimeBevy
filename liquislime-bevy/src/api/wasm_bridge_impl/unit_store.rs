use std::{
    ops::DerefMut,
    sync::{Arc, Mutex},
};

use wasm_bridge::{Config, Engine, Store};

#[derive(Clone)]
pub struct UnitStore {
    store: Arc<Mutex<Store<LiquislimeHost>>>,
}

impl std::fmt::Debug for UnitStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UnitStore cannot derive Debug")
    }
}

impl UnitStore {
    pub fn new() -> Self {
        let mut config = Config::new();
        config.wasm_component_model(true);

        let engine = Engine::new(&config).unwrap();
        let store = Store::new(&engine, LiquislimeHost);

        Self {
            store: Arc::new(Mutex::new(context)),
        }
    }

    pub fn store_mut(&self) -> impl DerefMut<Target = Store<LiquislimeHost>> + '_ {
        self.store.try_lock().unwrap()
    }
}
