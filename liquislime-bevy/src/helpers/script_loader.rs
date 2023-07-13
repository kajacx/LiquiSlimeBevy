use std::sync::Arc;

use bevy::{
    asset::{AssetLoader, LoadedAsset},
    prelude::*,
    reflect::{TypePath, TypeUuid},
};

use crate::{api::UnitModule, units::ScriptInstance};

#[derive(Clone, Debug, TypeUuid, TypePath)]
#[uuid = "39f0d1f8-a7eb-4eaa-887b-4f31a73c196e"]
pub struct ScriptAsset(pub Arc<ScriptInstance>);

#[derive(Clone, Debug, Default)]
pub struct ScriptLoader;

impl AssetLoader for ScriptLoader {
    fn extensions(&self) -> &[&str] {
        &["wasm", "zip"]
    }

    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let unit_module = UnitModule::from_bytes(bytes);
            let unit_instance = unit_module.instantiate();
            let script = ScriptInstance::new(unit_instance);
            let asset = ScriptAsset(Arc::new(script));
            load_context.set_default_asset(LoadedAsset::new(asset));
            Ok(())
        })
    }
}

pub struct AssetsGamePlugins;

impl Plugin for AssetsGamePlugins {
    fn build(&self, app: &mut App) {
        app.add_asset::<ScriptAsset>()
            .init_asset_loader::<ScriptLoader>();
    }
}
