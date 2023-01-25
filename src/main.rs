use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    ecs::system::Command,
    prelude::*,
    sprite::MaterialMesh2dBundle,
};
use components::{SlimeAmount, SlimeGrid, SlimeSource, Tile, TilePosition};
use systems::AllGamePlugins;

mod components;
mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(AllGamePlugins)
        .run();
}
