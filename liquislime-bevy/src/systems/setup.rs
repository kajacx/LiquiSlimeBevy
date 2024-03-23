use bevy::prelude::*;

use crate::assets::ScriptModule;
use crate::components::{FactionComponent, ScriptComponent, SlimeGrids};
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
                                plugin_filename: &str| {
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

        let script_component = get_plugin(plugin_filename, &asset_server);

        commands.spawn((
            FactionComponent::from(faction),
            TilePositionComponent::from(position),
            sprite,
            Building,
            script_component,
            unit_id,
        ));
    };

    create_unit(
        Faction::new(0),
        crate::api::TilePosition::new(2, 5),
        "tiles_grayscale/tile_0057.png",
        UnitId(1),
        "liquislime_slime_spawner_plugin.wasm",
    );

    create_unit(
        Faction::new(1),
        crate::api::TilePosition::new(7, 1),
        "tiles_grayscale/tile_0055.png",
        UnitId(2),
        "liquislime_slime_voider_plugin.wasm",
    );
}

fn get_plugin(plugin_filename: &str, asset_server: &Res<AssetServer>) -> ScriptComponent {
    let path = format!("plugins/{plugin_filename}");
    let handle: Handle<ScriptModule> = asset_server.load(path);

    ScriptComponent::new(handle)
}
