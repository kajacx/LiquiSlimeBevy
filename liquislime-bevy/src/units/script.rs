use std::{fmt::Debug, sync::Arc};

use bevy::prelude::{Assets, Handle};

use crate::api::*;
use crate::helpers::ScriptAsset;

#[derive(Debug)]
pub struct ScriptInstance {
    instance: UnitInstance,
}

impl ScriptInstance {
    pub fn new(instance: UnitInstance) -> Self {
        Self { instance }
    }

    pub fn update(&self, time_elapsed: TimeInterval) {
        self.instance.update(time_elapsed);
    }
}

#[derive(Debug)]
pub enum MaybeLoadedScript {
    Loaded(Arc<ScriptInstance>),
    Loading(Handle<ScriptAsset>), // TODO: add from impl
}

impl MaybeLoadedScript {
    pub fn new(handle: Handle<ScriptAsset>) -> Self {
        Self::Loading(handle)
    }

    pub fn try_get_script<'a>(
        &'a mut self,
        script_assets: &mut Assets<ScriptAsset>,
    ) -> Option<&'a ScriptInstance> {
        self.try_load(script_assets);

        match self {
            Self::Loaded(script) => Some(script),
            Self::Loading(_) => None,
        }
    }

    fn try_load(&mut self, script_assets: &mut Assets<ScriptAsset>) {
        let loaded_script = if let Self::Loading(handle) = self {
            script_assets.get(handle)
        } else {
            None
        };

        if let Some(script) = loaded_script {
            *self = Self::Loaded(script.0.clone());
        }
    }
}
