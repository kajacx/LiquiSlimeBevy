use crate::components::{
    FactionComponent, ScriptComponent, ScriptInstances, SelectorCursor, SlimeGrids,
};
use crate::{api::*, WORLD_HEIGHT, WORLD_WIDTH};
use crate::{
    components::UnitId,
    components::{Building, SlimeGrid, Tile, TilePositionComponent},
    resources,
};
use bevy::math::vec3;
use bevy::prelude::*;
use bevy::winit::WinitWindows;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use std::sync::Arc;
use winit::window::Icon;

pub struct GameSetupPlugin;

impl Plugin for GameSetupPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<resources::InputQueue>();
        app.init_resource::<resources::SelectedUnit>();
        app.init_resource::<resources::MouseState>();
        app.init_resource::<resources::GameWindowSpace>();

        app.add_systems(Startup, setup_camera);
        app.add_systems(Startup, spawn_tiles);
        app.add_systems(Startup, spawn_sources);
        app.add_systems(Startup, set_window_icon);
        app.add_systems(Startup, setup_selector);
    }
}

fn setup_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    let scale = 0.02f32;
    let position = 5f32;

    camera.transform.scale.x = scale;
    camera.transform.scale.y = scale;

    camera.transform.translation.x = position;
    camera.transform.translation.y = position;

    commands.spawn(camera);
}

fn spawn_tiles(mut commands: Commands) {
    commands.spawn(SlimeGrids::new(WORLD_WIDTH, WORLD_HEIGHT));

    for x in 0..WORLD_WIDTH {
        for y in 0..WORLD_HEIGHT {
            let position = ApiTilePosition {
                x: x as i32,
                y: y as i32,
            };
            let sprite = SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2 { x: 1f32, y: 1f32 }),
                    ..Default::default()
                },
                transform: Transform::from_translation(position.to_position_center().to_vec3(0.0)),
                ..Default::default()
            };

            commands.spawn((TilePositionComponent::from(position), sprite, Tile));
        }
    }
}

fn spawn_sources(mut commands: Commands, asset_server: Res<AssetServer>) {
    let spawner_script = Script::from_handle(
        "Slime spawner".to_owned(),
        asset_server.load("scripts/slime_spawner.wasm"),
    );

    let clicker_script = Script::from_handle(
        "Mouse clicker".to_owned(),
        asset_server.load("scripts/slime_clicker.wasm"),
    );

    commands.spawn(ScriptComponent(spawner_script.clone()));
    commands.spawn(ScriptComponent(clicker_script.clone()));

    let mut create_unit = move |faction: ApiFaction,
                                position: ApiTilePosition,
                                texture_file: &'static str,
                                unit_id: UnitId,
                                scripts: &[(&Script, SettingsValue)]| {
        let sprite = SpriteBundle {
            texture: asset_server.load(texture_file),
            sprite: Sprite {
                custom_size: Some(Vec2 {
                    x: 0.75f32,
                    y: 0.75f32,
                }),
                ..Default::default()
            },
            transform: Transform::from_translation(position.to_position_center().to_vec3(1.0)),
            ..Default::default()
        };

        let scripts = scripts
            .iter()
            .map(|(script, settings)| {
                Arc::new(ScriptInstance::new((*script).clone(), settings.clone()))
            })
            .collect::<Vec<_>>();

        commands.spawn((
            FactionComponent::from(faction),
            TilePositionComponent::from(position),
            sprite,
            Building,
            ScriptInstances(scripts),
            unit_id,
        ));
    };

    create_unit(
        ApiFaction::new(0),
        crate::api::ApiTilePosition::new(2, 5),
        "tiles_grayscale/tile_0057.png",
        UnitId(1),
        &[
            (&spawner_script, SettingsValue(rmpv::Value::from(100_i64))),
            (&clicker_script, SettingsValue(rmpv::Value::from(1000_i64))),
        ],
    );

    create_unit(
        ApiFaction::new(1),
        crate::api::ApiTilePosition::new(7, 1),
        "tiles_grayscale/tile_0055.png",
        UnitId(2),
        &[(&spawner_script, SettingsValue(rmpv::Value::from(120_i64)))],
    );
}

// From https://bevy-cheatbook.github.io/window/icon.html
fn set_window_icon(windows: NonSend<WinitWindows>) {
    if cfg!(target_arch = "wasm32") {
        return;
    }

    // here we use the `image` crate to load our icon data from a png file
    // this is not a very bevy-native solution, but it will do
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("assets/icon.png")
            .expect("failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

    // do it for all windows
    for window in windows.windows.values() {
        window.set_window_icon(Some(icon.clone()));
    }
}

fn setup_selector(mut commands: Commands, asset_server: Res<AssetServer>) {
    let sprite = SpriteBundle {
        texture: asset_server.load("icons/selector.png"),
        sprite: Sprite {
            custom_size: Some(Vec2 {
                x: 1.25f32,
                y: 1.25f32,
            }),
            ..Default::default()
        },
        transform: Transform::from_translation(vec3(0.0, 0.0, 2.0)),
        ..Default::default()
    };

    commands.spawn((sprite, SelectorCursor::default()));
}
