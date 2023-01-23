use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use components::{Building, TilePosition};

mod components;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(|| println!("Hello system"))
        .add_startup_system(spawn_buildings)
        .add_startup_system(setup)
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

    // Rectangle
    commands.spawn((
        Building {},
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(50.0, 100.0)),
                ..default()
            },
            ..default()
        },
    ));

    // Circle
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(50.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        transform: Transform::from_translation(Vec3::new(-100., 0., 0.)),
        ..default()
    });

    // Hexagon
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::RegularPolygon::new(50., 6).into()).into(),
        material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
        transform: Transform::from_translation(Vec3::new(100., 0., 0.)),
        ..default()
    });
}

//fn move_shapes(query: Query<&MaterialMesh2dBundle<ColorMaterial>>) {
fn move_shapes(mut query: Query<(&Building, &mut Transform)>) {
    for (_, mut transform) in &mut query {
        println!("{:?}", transform);
        transform.translation.x += 1f32;
    }
}
