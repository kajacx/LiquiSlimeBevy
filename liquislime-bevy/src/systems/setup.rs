use bevy::prelude::*;

use crate::{
    components::{Building, SlimeGrid, Tile, TilePositionComponent},
    resources::UnitScriptMap,
    units::{
        api_spec::types::{SlimeAmount, TilePosition},
        Script, UnitId,
    },
};

pub struct GameSetupPlugin;

impl Plugin for GameSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_resources);
        app.add_startup_system(setup_camera);
        app.add_startup_system(spawn_tiles(10, 10)); // TODO: Fixed world size
        app.add_startup_system(spawn_sources);
    }
}

fn setup_resources(mut commands: Commands) {
    //commands.insert_resource(UnitScriptMap::new());
}

fn setup_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    let scale = 0.02f32;
    let position = 5f32;

    //camera.transform = Transform::from_scale(Vec3::new(scale, scale, scale));
    camera.transform.scale.x = scale;
    camera.transform.scale.y = scale;

    //camera.transform.translation = Vec3::new(position, position, 0f32);
    camera.transform.translation.x = position;
    camera.transform.translation.y = position;

    commands.spawn(camera);
}

fn spawn_tiles(width: usize, height: usize) -> impl Fn(Commands, Res<AssetServer>) {
    move |mut commands, _asset_server| {
        // TODO: unused asset server?
        let mut slime_grid = SlimeGrid::new(width, height);

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

                let amount = SlimeAmount::from_integer(256 * 10 + 128);
                slime_grid.set_amount(x, y, amount);
            }
        }

        commands.spawn(slime_grid);
    }
}

fn spawn_sources(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    //mut unit_map: ResMut<UnitScriptMap>,
) {
    let mut unit_map = UnitScriptMap::new();

    create_spawner(
        &mut commands,
        &asset_server,
        TilePosition::new(2, 5),
        "tiles_grayscale/tile_0057.png",
        UnitId(1),
        "liquislime_slime_spawner_plugin.wasm",
        //&mut *unit_map,
        &mut unit_map,
    );

    create_spawner(
        &mut commands,
        &asset_server,
        TilePosition::new(7, 1),
        "tiles_grayscale/tile_0055.png",
        UnitId(2),
        "liquislime_slime_voider_plugin.wasm",
        //&mut *unit_map,
        &mut unit_map,
    );

    commands.insert_resource(unit_map);
}

fn create_spawner(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    position: TilePosition,
    texture_file: &'static str,
    unit_id: UnitId,
    plugin_filename: &'static str,
    unit_map: &mut UnitScriptMap,
) {
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

    unit_map.register_new_unit(unit_id, get_plugin(plugin_filename));

    commands.spawn((
        TilePositionComponent::from(position),
        sprite,
        Building,
        unit_id,
    ));
}

#[cfg(not(target_arch = "wasm32"))]
fn get_plugin(plugin_filename: &str) -> Script {
    let path = format!("assets/plugins/{plugin_filename}");
    Script::from_plugin_path(&path)
}

#[cfg(target_arch = "wasm32")]
fn get_plugin(plugin_filename: &str) -> Script {
    // TODO: load via an url request?

    let bytes = if plugin_filename == "liquislime_slime_spawner_plugin.wasm" {
        include_bytes!("../../assets/plugins/liquislime_slime_spawner_plugin.wasm").as_ref()
    } else if plugin_filename == "liquislime_slime_voider_plugin.wasm" {
        include_bytes!("../../assets/plugins/liquislime_slime_voider_plugin.wasm").as_ref()
    } else {
        panic!("Unknown plugin: {}", plugin_filename)
    };

    Script::from_bytes(bytes)
}
