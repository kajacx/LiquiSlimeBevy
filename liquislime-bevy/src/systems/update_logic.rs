use bevy::prelude::*;

use crate::components::SlimeGrid;

pub struct UpdateLogicPlugin;

impl Plugin for UpdateLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::Update, spread_slime);
    }
}

fn spread_slime(mut query: Query<&mut SlimeGrid>) {
    let mut slime_grid = query
        .get_single_mut()
        .expect("Slime Grid should have been created");

    slime_grid.prepare_slime_spread();
    slime_grid.spread_slime();
}
