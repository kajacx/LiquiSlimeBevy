use bevy::{ecs::system::Command, prelude::*, sprite::MaterialMesh2dBundle};
use components::{Building, SlimeAmount, TilePosition};

mod components;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(|| println!("Hello system"))
        .add_startup_system(spawn_buildings)
        .add_startup_system(setup)
        .add_startup_system(spawn_tiles(10, 10))
        .add_system(list_buildings)
        .add_system(move_shapes)
        .run();
}

fn spawn_buildings(mut commands: Commands) {
    commands.spawn((Building {}, TilePosition { x: 32, y: -6 }));
    commands.spawn((Building {}, TilePosition { x: 15, y: 14 }));
}

fn list_buildings(query: Query<&TilePosition, With<Building>>) {
    for building in query.iter() {
        println!("Hello? {:?}", building);
    }
}
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_tiles(width: usize, height: usize) -> impl Fn(Commands) {
    move |mut commands| {
        for x in 0..width {
            for y in 0..height {
                let position = TilePosition {
                    x: x as i32,
                    y: y as i32,
                };
                let amount = SlimeAmount((x + y * 10) as u64);
                let sprite = SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2 { x: 10f32, y: 10f32 }),
                        ..Default::default()
                    },
                    transform: Transform::from_translation(Vec3::new(
                        (x * 10) as f32,
                        (y * 10) as f32,
                        0f32,
                    )),
                    ..Default::default()
                };

                commands.spawn((position, amount, sprite));
            }
        }
    }
}

//fn move_shapes(query: Query<&MaterialMesh2dBundle<ColorMaterial>>) {
fn move_shapes(mut query: Query<(&Building, &mut Transform)>) {
    for (_, mut transform) in &mut query {
        println!("{:?}", transform);
        transform.translation.x += 1f32;
    }
}
