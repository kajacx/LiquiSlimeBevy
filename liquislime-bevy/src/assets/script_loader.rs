use super::ScriptAsset;
use crate::api::{LoadedScript, Script};
use bevy::utils::thiserror::Error;
use bevy::{
    asset::io::Reader,
    asset::{AssetLoader, AsyncReadExt, LoadedAsset},
    prelude::*,
};
use try_lock::TryLock;

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
    type Asset = ScriptAsset;
    type Settings = ();
    type Error = CustomError;

    fn extensions(&self) -> &[&str] {
        &["wasm"]
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
            Ok(ScriptAsset { bytes })
        })
    }
}

pub struct AssetsGamePlugin;

impl Plugin for AssetsGamePlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<ScriptAsset>()
            .init_asset_loader::<ScriptLoader>();
    }
}
