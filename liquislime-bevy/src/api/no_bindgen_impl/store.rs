use super::{get_engine, ScriptImpl};
use once_cell::sync::Lazy;
use std::ops::{Deref, DerefMut};
use try_lock::{Locked, TryLock};
use wasm_bridge::Store;

static STORE: Lazy<TryLock<Store<StoreData>>> = Lazy::new(|| {
    TryLock::new(Store::new(
        get_engine(),
        StoreData {
            current_script: ScriptImpl::new(),
        },
    ))
});

pub fn get_store(name: &'static str) -> impl DerefMut<Target = Store<StoreData>> {
    println!("LOCKING {}", name);
    let lock = STORE.try_lock().unwrap();
    println!("LOCKED {}", name);
    NamedDeref(name, lock)
}

#[derive(Debug)]
pub struct StoreData {
    pub current_script: ScriptImpl,
}

pub struct NamedDeref(&'static str, Locked<'static, Store<StoreData>>);

impl Deref for NamedDeref {
    type Target = Store<StoreData>;

    fn deref(&self) -> &Self::Target {
        &*self.1
    }
}

impl DerefMut for NamedDeref {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.1
    }
}

impl Drop for NamedDeref {
    fn drop(&mut self) {
        println!("UNLOCKING {}", self.0);
    }
}
