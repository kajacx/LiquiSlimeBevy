use once_cell::sync::Lazy;
use std::future::Future;
use wasm_bridge::Engine;

static ENGINE: Lazy<Engine> = Lazy::new(Engine::default);

pub fn get_engine() -> &'static Engine {
    &ENGINE
}
