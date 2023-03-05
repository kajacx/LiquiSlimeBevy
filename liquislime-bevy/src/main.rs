use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use systems::AllGamePlugins;
use wasm_bindgen::prelude::*;

mod components;
mod helpers;
mod resources;
mod systems;
mod units;

//#[wasm_bindgen(memory = "16")]

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(AllGamePlugins)
        .run();
}
