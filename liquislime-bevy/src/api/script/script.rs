use super::{GameImports, LiquislimeImports, LoadedScript, ScriptInstance};
use crate::{
    api::{ScriptImpl, SettingsDescription, SettingsValue},
    assets::ScriptAsset,
};
use anyhow::Result;
use bevy::{prelude::*, render::settings};
use std::{error::Error, fmt::Display, sync::Arc};
use try_lock::TryLock;

#[derive(Debug, Clone)]
pub struct Script(Arc<ScriptInner>);

#[derive(Debug)]
struct ScriptInner {
    name: String,
    state: TryLock<ScriptState>,
}

#[derive(Debug)]
enum ScriptState {
    Loaded(LoadedScript),
    AssetLoading(Handle<ScriptAsset>),
}

impl Script {
    pub fn from_bytes(name: String, bytes: &[u8], imports: impl LiquislimeImports) -> Result<Self> {
        Ok(Self(Arc::new(ScriptInner {
            name,
            state: TryLock::new(ScriptState::Loaded(LoadedScript::from_bytes(
                bytes, imports,
            )?)),
        })))
    }

    pub fn from_handle(name: String, handle: Handle<ScriptAsset>) -> Self {
        Self(Arc::new(ScriptInner {
            name,
            state: TryLock::new(ScriptState::AssetLoading(handle)),
        }))
    }

    pub fn try_load(&self, script_assets: &Assets<ScriptAsset>) -> ScriptStatus {
        let mut lock = self.0.state.try_lock().unwrap();

        let (new, status) = match &*lock {
            ScriptState::AssetLoading(handle) => {
                if let Some(asset) = script_assets.get(handle) {
                    (
                        Some(ScriptState::Loaded(
                            // TODO: Will always use GameImports unless testing, but it's not great
                            LoadedScript::from_bytes(&asset.bytes, GameImports)
                                .expect("TODO: user error"),
                        )),
                        ScriptStatus::Loaded,
                    )
                } else {
                    (None, ScriptStatus::NotLoaded)
                }
            }
            ScriptState::Loaded(_) => (None, ScriptStatus::Loaded),
        };

        if let Some(new) = new {
            *lock = new;
        };

        status
    }

    pub fn with_loaded<T>(
        &self,
        callback: impl FnOnce(&LoadedScript) -> T,
    ) -> Result<T, ScriptNotLoadedError> {
        let lock = self.0.state.try_lock().unwrap();
        match &*lock {
            ScriptState::Loaded(loaded) => Ok(callback(loaded)),
            ScriptState::AssetLoading(_) => Err(ScriptNotLoadedError),
        }
    }

    pub fn name(&self) -> &str {
        &self.0.name
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScriptStatus {
    Loaded,
    NotLoaded,
}

#[derive(Debug, Clone, Copy)]
pub struct ScriptNotLoadedError;

impl Error for ScriptNotLoadedError {}

impl Display for ScriptNotLoadedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Script not loaded")
    }
}
