use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

use wasmtime::{
    component::{Component, Linker},
    Config, Engine, Store,
};

wasmtime::component::bindgen!({
    path: "../protocol.wit",
    world: "liquislime-unit"
});

#[derive(Debug)]
pub struct LiquislimeHost;

impl LiquislimeUnitImports for LiquislimeHost {
    fn get_own_position(&mut self) -> wasmtime::Result<TilePosition> {
        todo!("import get own")
    }

    fn add_slime_amount(
        &mut self,
        position: TilePosition,
        amount: SlimeAmount,
    ) -> wasmtime::Result<()> {
        todo!("import add slime")
    }
}

pub struct UnitModule {
    store: Arc<Mutex<Store<LiquislimeHost>>>,
    linker: Linker<LiquislimeHost>,
    component: Component,
}

impl Debug for UnitModule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UnitModule cannot derive Debug")
    }
}

pub struct UnitInstance {
    store: Arc<Mutex<Store<LiquislimeHost>>>,
    instance: LiquislimeUnit,
}

impl Debug for UnitInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UnitInstance cannot derive Debug")
    }
}

impl UnitModule {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        // TODO: share same store between multiple modules
        let mut config = Config::new();
        config.wasm_component_model(true);

        let engine = Engine::new(&config).unwrap();
        let store = Store::new(&engine, LiquislimeHost);

        let component = Component::new(&store.engine(), &bytes).unwrap();

        let mut linker = Linker::new(store.engine());
        LiquislimeUnit::add_to_linker(&mut linker, |state| state).unwrap();

        Self {
            store: Arc::new(Mutex::new(store)),
            linker,
            component,
        }
    }

    pub fn instantiate(&self) -> UnitInstance {
        let (instance, _) = LiquislimeUnit::instantiate(
            &mut *self.store.try_lock().unwrap(),
            &self.component,
            &self.linker,
        )
        .unwrap();
        UnitInstance {
            store: self.store.clone(),
            instance,
        }
    }
}

impl UnitInstance {
    fn update(&self, time_elapsed: crate::api::TimeInterval) {
        let time_elapsed = TimeInterval {
            fragments: time_elapsed.0,
        };
        self.instance
            .call_update(&mut *self.store.try_lock().unwrap(), time_elapsed)
            .unwrap();
    }
}
