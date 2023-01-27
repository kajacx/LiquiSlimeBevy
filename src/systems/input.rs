use bevy::{input::mouse::MouseButtonInput, prelude::*};

use crate::components::{Building, MoveOnClick, SlimeGrid, Tile, TilePosition};

pub struct GameInputPlugin;

impl Plugin for GameInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::First, move_sources);
    }
}

fn move_sources(
    mut spawners: Query<(&mut TilePosition, &MoveOnClick)>,
    mut mouse_input: Res<Input<MouseButton>>,
) {
    for (mut spawner, move_on) in &mut spawners {
        if mouse_input.pressed(move_on.mouse_button) {
            spawner.x = (spawner.x + 1) % 10; // TODO: hardwired level width
        }
    }
}
