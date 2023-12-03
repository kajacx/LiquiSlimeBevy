// use bevy::render::render_resource::encase::internal::Reader;
use bevy::utils::thiserror::Error;

use bevy::{
    asset::io::Reader,
    asset::{AssetLoader, AsyncReadExt, LoadedAsset},
    prelude::*,
};

use crate::api::UnitModule;

use super::ScriptModule;

#[derive(Clone, Debug, Default)]
pub struct ScriptLoader;

#[derive(Debug, Error)]
pub struct CustomError;

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("CustomError")
    }
}

impl AssetLoader for ScriptLoader {
    type Asset = ScriptModule;
    type Settings = ();
    type Error = CustomError;

    fn extensions(&self) -> &[&str] {
        &["wasm", "zip"]
    }

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        _load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await.unwrap();
            let unit_module = UnitModule::from_bytes(&bytes).await;
            let asset = ScriptModule::new("TODO: module name".into(), unit_module);
            // load_context.set_default_asset(LoadedAsset::new(asset));
            Ok(asset)
        })
    }
}

pub struct AssetsGamePlugins;

impl Plugin for AssetsGamePlugins {
    fn build(&self, app: &mut App) {
        app.init_asset::<ScriptModule>()
            .init_asset_loader::<ScriptLoader>();
    }
}
