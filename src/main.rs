use bevy::prelude::*;

mod components;

fn main() {
    App::new().add_system(|| println!("Hello system")).run();
}
