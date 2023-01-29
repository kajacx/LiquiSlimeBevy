use bevy::prelude::*;

use crate::components::{
    Building, MoveOnClick, SlimeAmount, SlimeGrid, SlimeSource, Tile, TilePosition,
};

pub struct GameSetupPlugin;

impl Plugin for GameSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_camera);
        app.add_startup_system(spawn_tiles(10, 10));
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
    move |mut commands, asset_server| {
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
                    transform: Transform::from_translation(position.to_centered_vec(0.0)),
                    ..Default::default()
                };

                commands.spawn((position, sprite, Tile));

                let amount = SlimeAmount::from_integer((x + y * 20) as i64);
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
        SlimeAmount::from_integer(10),
        TilePosition::new(2, 5),
        MouseButton::Left,
        "tiles_grayscale/tile_0057.png",
    );

    create_spawner(
        &mut commands,
        &asset_server,
        SlimeAmount::from_integer(-10),
        TilePosition::new(7, 1),
        MouseButton::Right,
        "tiles_grayscale/tile_0055.png",
    );
}

fn create_spawner(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    amount: SlimeAmount,
    position: TilePosition,
    move_button: MouseButton,
    texture_file: &'static str,
) {
    let spawner = SlimeSource { amount };

    let sprite = SpriteBundle {
        texture: asset_server.load(texture_file),
        sprite: Sprite {
            custom_size: Some(Vec2 { x: 1f32, y: 1f32 }),
            ..Default::default()
        },
        transform: Transform::from_translation(position.to_centered_vec(1.0)),
        ..Default::default()
    };

    let move_on = MoveOnClick {
        mouse_button: move_button,
    };

    commands.spawn((spawner, position, sprite, move_on, Building));
}
