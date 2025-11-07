use bevy::prelude::*;

use crate::{
    api::ApiPosition,
    components::{MovementComponent, SlimeGrids, TilePositionComponent},
    helpers::Phase,
};

pub struct UpdateLogicPlugin;

impl Plugin for UpdateLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spread_slime.in_set(Phase::GameUpdate));
        app.add_systems(Update, move_units.in_set(Phase::GameUpdate));
    }
}

fn spread_slime(mut query: Query<&mut SlimeGrids>) {
    let mut slime_grid = query
        .get_single_mut()
        .expect("Slime Grid should have been created");

    slime_grid.prepare_slime_spread();
    slime_grid.spread_slime();
    slime_grid.annihilate_slime();
}

fn move_units(mut query: Query<(&mut Transform, &mut MovementComponent)>) {
    for (mut position, mut movement) in query.iter_mut() {
        if let Some(target_position) = &movement.moving_to {
            let direction = ApiPosition {
                x: target_position.x - position.translation.x,
                y: target_position.y - position.translation.y,
            };

            let distance = (direction.x.powi(2) + direction.y.powi(2)).sqrt();

            if distance <= movement.movement_speed {
                position.translation.x = target_position.x;
                position.translation.y = target_position.y;
                movement.moving_to = None;
            } else {
                let normalized_direction = ApiPosition {
                    x: direction.x / distance,
                    y: direction.y / distance,
                };

                // TODO: Update time?
                position.translation.x += normalized_direction.x * movement.movement_speed;
                position.translation.y += normalized_direction.y * movement.movement_speed;
            }
        }
    }
}
