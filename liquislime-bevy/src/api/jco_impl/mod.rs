use std::fmt::Debug;

use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::Window;

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
    pub async fn from_bytes(bytes: &[u8]) -> Self {
        let plugin = std::str::from_utf8(bytes).unwrap();
        let path = format!("/webserver/assets/plugins/{plugin}-jco/slime-voider-componenet.js");
        // let request = web_sys::Request::new_with_str(&path);

        let window = web_sys::window().unwrap();
        let resp_value = JsFuture::from(window.fetch_with_str(&path)).await.unwrap();

        todo!("Lets see: {resp_value:?}")
    }

    pub async fn instantiate(&self) -> UnitInstance {
        todo!("instantiate")
    }
}

impl UnitInstance {
    pub fn update(&self, time_elapsed: crate::api::TimeInterval) {
        todo!("update")
    }
}
