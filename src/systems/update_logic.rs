use bevy::prelude::*;

pub struct UpdateLogicPlugin;

impl Plugin for UpdateLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(CoreStage::PreUpdate, spawn_slime);
        app.add_system_to_stage(CoreStage::Update, spread_slime);
    }
}

fn spread_slime(mut query: Query<&mut SlimeGrid>) {
    let mut slime_grid = query.single_mut();
    slime_grid.prepare_slime_spread();
    slime_grid.spread_slime();
}

fn spawn_slime(
    mut grid_query: Query<&mut SlimeGrid>,
    spawner_query: Query<(&SlimeSource, &TilePosition)>,
) {
    let mut slime_grid = grid_query.single_mut();
    for (spawner, position) in &spawner_query {
        slime_grid.add_amount(position.x as usize, position.y as usize, spawner.amount);
    }
}
