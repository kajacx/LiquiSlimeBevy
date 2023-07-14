use bevy::prelude::*;

use crate::{assets::ScriptModule, units::ScriptInstance};

#[derive(Debug, Component)]
pub struct ScriptComponent {
    instance: Option<ScriptInstance>,
    handle: Handle<ScriptModule>,
}

impl ScriptComponent {
    pub fn new(handle: Handle<ScriptModule>) -> Self {
        Self {
            instance: None,
            handle,
        }
    }

    pub fn try_load(&mut self, script_assets: &Assets<ScriptModule>) {
        if self.instance.is_none() {
            let module = script_assets.get(&self.handle);
            if let Some(module) = module {
                self.instance = Some(module.instantiate());
            }
        }
    }

    pub fn instance(&self) -> Option<&ScriptInstance> {
        self.instance.as_ref()
    }
}
