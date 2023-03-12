use bevy::{
    asset::{AssetLoader, LoadedAsset},
    reflect::TypeUuid,
};

#[derive(Clone, Debug, TypeUuid)]
#[uuid = "39f0d1f8-a7eb-4eaa-887b-4f31a73c196e"]
pub struct RawBytes(pub Vec<u8>);

#[derive(Clone, Debug, Default)]
pub struct RawBytesLoader;

impl AssetLoader for RawBytesLoader {
    fn extensions(&self) -> &[&str] {
        // TODO: Would making this a wasm-specific asset loader be more idiomatic?
        &["wasm"]
    }

    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let bytes_vec: Vec<u8> = bytes.into();
            load_context.set_default_asset(LoadedAsset::new(RawBytes(bytes_vec)));
            Ok(())
        })
    }
}
