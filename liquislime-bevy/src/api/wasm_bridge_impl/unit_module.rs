use wasm_bridge::component::*;

use super::*;

pub struct UnitModule {
    store: UnitStore,
    linker: Linker<LiquislimeHost>,
    component: Component,
}

impl UnitModule {
    pub async fn from_bytes(bytes: &[u8]) -> Self {
        let store = UnitStore::new();
        let engine = store.store_mut().engine().clone();

        let component = component_new_async(&engine, &bytes).await.unwrap();

        let mut linker = Linker::new(&engine);
        LiquislimeUnit::add_to_linker(&mut linker, |state| state).unwrap();

        Self {
            store,
            linker,
            component,
        }
    }

    pub fn instantiate(&self) -> UnitInstance {
        let (instance, _) = LiquislimeUnit::instantiate(
            &mut *self.store.store_mut(),
            &self.component,
            &self.linker,
        )
        .expect("TODO: user error");

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
