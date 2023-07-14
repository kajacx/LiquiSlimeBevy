use bevy::prelude::*;

use crate::{
    assets::ScriptModule,
    components::{ScriptComponent, SlimeGrid},
    helpers::Phase,
};

pub struct AssetLoadPlugin;

impl Plugin for AssetLoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, load_assets.in_set(Phase::AssetLoad));
    }
}

fn load_assets(
    mut components: Query<&mut ScriptComponent>,
    mut asset_server: Res<Assets<ScriptModule>>,
) {
    for mut component in components.iter_mut() {
        component.try_load(asset_server.as_ref());
    }
}
