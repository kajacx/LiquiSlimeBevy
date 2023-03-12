use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use helpers::{RawBytes, RawBytesLoader};
use systems::AllGamePlugins;

mod components;
mod helpers;
mod resources;
mod systems;
mod units;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_asset::<RawBytes>()
        .init_asset_loader::<RawBytesLoader>() // TODO: can this be moved to a plugin?
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(AllGamePlugins)
        .run();
}
