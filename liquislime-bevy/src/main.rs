#![allow(unused)]
#![feature(async_closure)]

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use systems::AllGamePlugins;

mod api;
mod assets;
mod components;
mod helpers;
mod resources;
mod systems;
mod units;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Liquislime!".into(),
                resolution: (800.0, 600.0).into(),
                canvas: Some("#game_render".into()),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(AllGamePlugins)
        .run();
}
