use std::fmt::Debug;

use wasm_bindgen::JsValue;

#[derive(Debug)]
pub struct UnitModule {
    module: JsValue,
}

#[derive(Debug)]
pub struct UnitInstance {
    instance: JsValue,
}

// SAFETY: Bevy says it runs on only one "thread" (web worker) on the web
unsafe impl Send for UnitInstance {}
unsafe impl Sync for UnitInstance {}

impl UnitModule {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        todo!("from bytes")
    }

    pub fn instantiate(&self) -> UnitInstance {
        todo!("instantiate")
    }
}

impl UnitInstance {
    pub fn update(&self, time_elapsed: crate::api::TimeInterval) {
        todo!("update")
    }
}
