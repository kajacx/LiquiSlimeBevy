use bevy::{
    asset::{AssetLoader, LoadedAsset},
    prelude::*,
};

use crate::api::UnitModule;

use super::ScriptModule;

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
            let asset = ScriptModule::new("TODO: module name".into(), unit_module);
            load_context.set_default_asset(LoadedAsset::new(asset));
            Ok(())
        })
    }
}

pub struct AssetsGamePlugins;

impl Plugin for AssetsGamePlugins {
    fn build(&self, app: &mut App) {
        app.add_asset::<ScriptModule>()
            .init_asset_loader::<ScriptLoader>();
    }
}
