use wasmtime::{
    component::{Component, Linker},
    Config, Engine, Store,
};

wasmtime::component::bindgen!({
    path: "../protocol.wit",
    world: "liquislime-unit"
});

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
    store: Arc<Mutex<Store>>,
    linker: Linker,
    component: Component,
}

pub struct UnitInstance {
    store: Arc<Mutex<Store>>,
    instance: LiquislimeUnit;
}

impl UnitModule {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut config = Config::new();
        config.wasm_component_model(true);
    
        let engine = Engine::new(&config)?;
        let store = Store::new(&engine, State::default());

        let component = Component::new(&store.engine(), &bytes).expect("create component");
    
        let mut linker = Linker::new(store.engine());
        MyWorld::add_to_linker(&mut linker, |state| state)?;

        Self {
            store, linker, component
        }
    }

    pub fn instantiate(&self) -> UnitInstance {
        let (instance, _) = LiquislimeUnit::instantiate(&mut *self.store.try_lock().unwrap(), &self.component, &self.linker).unwrap();
        UnitInstance {
            store: self.store.clone(),
            instance
        }
    }
}

impl UnitInstance {
    fn update(&self, time_elapsed: crate::api::TimeInterval) {
        let time_elapsed = TimeInterval {fragments: time_elapsed.0};
        self.instance.call_update(, *self.store.try_lock().unwrap(), time_elapsed).unwrap();
    }
}

