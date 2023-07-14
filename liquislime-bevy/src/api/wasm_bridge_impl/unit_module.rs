use wasm_bridge::component::*;

use super::*;

pub struct UnitModule {
    store: UnitStore,
    linker: Linker<LiquislimeHost>,
    component: Component,
}

impl UnitModule {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let store = UnitStore::new();
        let store_obj = store.store_mut();

        let component = new_universal_component(&store_obj.engine(), &bytes).unwrap();

        let mut linker = Linker::new(store_obj.engine());
        LiquislimeUnit::add_to_linker(&mut linker, |state| state).unwrap();

        drop(store_obj);
        Self {
            store,
            linker,
            component,
        }
    }

    pub fn instantiate(&self) -> UnitInstance {
        // TODO: wasm file might be bad, this should be user error
        let (instance, _) = LiquislimeUnit::instantiate(
            &mut *self.store.store_mut(),
            &self.component,
            &self.linker,
        )
        .unwrap();

        UnitInstance::new(self.store.clone(), instance)
    }
}

// SAFETY: Bevy says it runs on only one "thread" (web worker) on the web
#[cfg(target_arch = "wasm32")]
unsafe impl Send for UnitModule {}
#[cfg(target_arch = "wasm32")]
unsafe impl Sync for UnitModule {}

impl std::fmt::Debug for UnitModule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UnitModule cannot derive Debug")
    }
}
