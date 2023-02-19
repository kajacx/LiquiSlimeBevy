use bevy::prelude::*;

use crate::{
    components::{Building, SlimeGrid, Tile},
    units::{
        api_spec::types::{SlimeAmount, TilePosition},
        register_new_unit, Script, UnitId,
    },
};

pub struct GameSetupPlugin;

impl Plugin for GameSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_camera);
        app.add_startup_system(spawn_tiles(10, 10)); // TODO: Fixed world size
        app.add_startup_system(spawn_sources);
    }
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

                commands.spawn((position, sprite, Tile));

                let amount = SlimeAmount::from_integer(256 * 10 + 128);
                slime_grid.set_amount(x, y, amount);
            }
        }

        commands.spawn(slime_grid);
    }
}

fn spawn_sources(mut commands: Commands, asset_server: Res<AssetServer>) {
    create_spawner(
        &mut commands,
        &asset_server,
        TilePosition::new(2, 5),
        "tiles_grayscale/tile_0057.png",
        UnitId(1),
        "../liquislime-plugins/slime-spawner/target/wasm32-unknown-unknown/debug/liquislime_slime_spawner_plugin.wasm"
    );

    create_spawner(
        &mut commands,
        &asset_server,
        TilePosition::new(7, 1),
        "tiles_grayscale/tile_0055.png",
        UnitId(2),
        "../liquislime-plugins/slime-voider/target/wasm32-unknown-unknown/debug/liquislime_slime_voider_plugin.wasm"
    );
}

fn create_spawner(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    position: TilePosition,
    texture_file: &'static str,
    unit_id: UnitId,
    plugin_path: &'static str,
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

    register_new_unit(unit_id, Script::from_plugin_path(plugin_path));

    commands.spawn((position, sprite, Building, unit_id));
}
