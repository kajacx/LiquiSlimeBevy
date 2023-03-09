use bevy::prelude::*;

use crate::components::SlimeGrid;

pub struct UpdateLogicPlugin;

impl Plugin for UpdateLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spread_slime.in_base_set(CoreSet::Update));
    }
}

fn spread_slime(mut query: Query<&mut SlimeGrid>) {
    let mut slime_grid = query
        .get_single_mut()
        .expect("Slime Grid should have been created");

    slime_grid.prepare_slime_spread();
    slime_grid.spread_slime();
}
