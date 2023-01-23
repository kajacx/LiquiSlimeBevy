use bevy::prelude::*;
use components::{Building, TilePosition};

mod components;

fn main() {
    App::new()
        .add_system(|| println!("Hello system"))
        .add_startup_system(spawn_buildings)
        .add_system(list_buildings)
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
