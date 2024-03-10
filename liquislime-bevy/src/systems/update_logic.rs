use bevy::prelude::*;

use crate::{components::SlimeGrids, helpers::Phase};

pub struct UpdateLogicPlugin;

impl Plugin for UpdateLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spread_slime.in_set(Phase::GameUpdate));
    }
}

fn spread_slime(mut query: Query<&mut SlimeGrids>) {
    let mut slime_grid = query
        .get_single_mut()
        .expect("Slime Grid should have been created");

    slime_grid.prepare_slime_spread();
    slime_grid.spread_slime();
}
