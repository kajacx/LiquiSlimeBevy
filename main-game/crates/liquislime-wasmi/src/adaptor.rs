use liquislime_core::*;
use wasmi_component::{anyhow, wasmi::Engine, Linker, Store};

use crate::{
    bindings::{add_adaptor_to_linker, instantiate_adaptor_world, AdaptorExports},
    store_data::StoreData,
};

pub struct WasmiAdaptor {
    store: Store<StoreData>,
    exports: AdaptorExports,
}

impl WasmiAdaptor {
    pub fn new(bytes: &[u8]) -> anyhow::Result<Self> {
        let mut store = Store::new(&Engine::default(), StoreData::new());
        let component = store.new_component(bytes)?;

        let mut linker = Linker::<StoreData>::new(&store.engine());
        add_adaptor_to_linker(&mut linker)?;
        wasmi_component_wasi::add_wasi_p2_to_linker(&mut linker)?;

        let exports = instantiate_adaptor_world(&mut store, &linker, &component)?;

        // TODO: start function?
        // let start = instance.get_export(&store, "_start");
        // if let Some(start) = start {
        //     start
        //         .into_func()
        //         .expect("TODO: func")
        //         .typed::<(), ()>(&store)
        //         .expect("TODO: typed")
        //         .call(&mut store, ())
        //         .expect("TODO: Failed to invoke '_start' export");
        // }

        Ok(Self { store, exports })
    }
}

impl BehaviourAdaptor for WasmiAdaptor {
    fn update(&mut self, game_interaction: &mut GameInteraction, time_elapsed: TimeInterval) {
        self.store.data_mut().set_interaction(game_interaction);

        println!("Calling update");

        let result = self
            .exports
            .call_update(&mut self.store, time_elapsed.to_seconds());

        if let Err(error) = result {
            println!("Error when calling update: {error}");
        }

        self.store.data_mut().clear_interaction();
    }
}
