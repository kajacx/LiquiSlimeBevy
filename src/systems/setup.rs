use bevy::prelude::*;

use crate::components::{SlimeAmount, SlimeGrid, SlimeSource, TilePosition};

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

    camera.transform = Transform::from_scale(Vec3::new(scale, scale, scale));
    camera.transform.translation = Vec3::new(position, position, 0f32);

    commands.spawn(camera);
}

fn spawn_tiles(width: usize, height: usize) -> impl Fn(Commands) {
    move |mut commands| {
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
                    transform: Transform::from_translation(Vec3::new((x) as f32, (y) as f32, 0f32)),
                    ..Default::default()
                };

                commands.spawn((position, sprite));

                let amount = SlimeAmount::from_integer((x + y * 20) as i64);
                slime_grid.set_amount(x, y, amount);
            }
        }

        commands.spawn(slime_grid);
    }
}

fn spawn_sources(mut commands: Commands) {
    let spawner = SlimeSource {
        amount: SlimeAmount::from_integer(10),
    };
    let position = TilePosition::new(2, 5);
    commands.spawn((spawner, position));

    let spawner = SlimeSource {
        amount: SlimeAmount::from_integer(-10),
    };
    let position = TilePosition::new(6, 8);
    commands.spawn((spawner, position));
}
