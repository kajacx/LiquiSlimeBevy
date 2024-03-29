use bevy::prelude::*;

use crate::{
    assets::ScriptModule,
    components::{ScriptsComponent, SlimeGrid},
    helpers::Phase,
};

pub struct AssetLoadPlugin;

impl Plugin for AssetLoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, load_assets.in_set(Phase::AssetLoad));
    }
}

fn load_assets(
    mut components: Query<&mut ScriptsComponent>,
    mut asset_server: Res<Assets<ScriptModule>>,
) {
    for mut component in components.iter_mut() {
        component
            .0
            .iter_mut()
            .for_each(|(script, settings)| script.try_load(asset_server.as_ref(), settings));
    }
}
