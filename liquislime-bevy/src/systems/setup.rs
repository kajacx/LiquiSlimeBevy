use bevy::prelude::*;
use bevy::winit::WinitWindows;
use winit::window::Icon;

use crate::assets::ScriptModule;
use crate::components::{FactionComponent, ScriptComponent, ScriptsComponent, SlimeGrids};
use crate::{api::*, WORLD_HEIGHT, WORLD_WIDTH};
use crate::{
    components::{Building, SlimeGrid, Tile, TilePositionComponent},
    units::UnitId,
};

pub struct GameSetupPlugin;

impl Plugin for GameSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
        app.add_systems(Startup, spawn_tiles(WORLD_WIDTH, WORLD_HEIGHT));
        app.add_systems(Startup, spawn_sources);
        app.add_systems(Startup, set_window_icon);
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

fn spawn_tiles(width: usize, height: usize) -> impl Fn(Commands, Res<AssetServer>) {
    move |mut commands, _asset_server| {
        let mut slime_grid = SlimeGrids::new(width, height);

        for x in 0..width {
            for y in 0..height {
                let position = TilePosition {
                    x: x as i32,
                    y: y as i32,
                };
                let sprite = SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2 { x: 1f32, y: 1f32 }),
                        ..Default::default()
                    },
                    transform: Transform::from_translation(
                        position.to_position_center().to_vec3(0.0),
                    ),
                    ..Default::default()
                };

                commands.spawn((TilePositionComponent::from(position), sprite, Tile));
            }
        }

        commands.spawn(slime_grid);
    }
}

fn spawn_sources(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut create_unit = move |faction: Faction,
                                position: TilePosition,
                                texture_file: &'static str,
                                unit_id: UnitId,
                                plugins: &[(&str, Settings)]| {
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

        let scripts_component = plugins
            .iter()
            .map(|(plugin_filename, settings)| {
                (get_plugin(plugin_filename, &asset_server), settings.clone())
            })
            .collect::<Vec<_>>();

        commands.spawn((
            FactionComponent::from(faction),
            TilePositionComponent::from(position),
            sprite,
            Building,
            ScriptsComponent(scripts_component),
            unit_id,
        ));
    };

    create_unit(
        Faction::new(0),
        crate::api::TilePosition::new(2, 5),
        "tiles_grayscale/tile_0057.png",
        UnitId(1),
        &[
            (
                "liquislime_slime_spawner_plugin.wasm",
                Settings(
                    serde_json::json!({
                        "amount": SlimeAmount::from_integer(100)
                    })
                    .to_string(),
                ),
            ),
            (
                "liquislime_slime_clicker_plugin.wasm",
                Settings(
                    serde_json::json!({
                        "amount": SlimeAmount::from_integer(2000)
                    })
                    .to_string(),
                ),
            ),
        ],
    );

    create_unit(
        Faction::new(1),
        crate::api::TilePosition::new(7, 1),
        "tiles_grayscale/tile_0055.png",
        UnitId(2),
        &[(
            "liquislime_slime_spawner_plugin.wasm",
            Settings(
                serde_json::json!({
                    "amount": SlimeAmount::from_integer(150)
                })
                .to_string(),
            ),
        )],
    );
}

fn get_plugin(plugin_filename: &str, asset_server: &Res<AssetServer>) -> ScriptComponent {
    let path = format!("plugins/{plugin_filename}");
    let handle: Handle<ScriptModule> = asset_server.load(path);

    ScriptComponent::new(handle)
}

// From https://bevy-cheatbook.github.io/window/icon.html
fn set_window_icon(
    // we have to use `NonSend` here
    windows: NonSend<WinitWindows>,
) {
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
