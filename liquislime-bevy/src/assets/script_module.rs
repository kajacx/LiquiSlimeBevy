use bevy::prelude::*;

use crate::{
    api::{SettingsValue, UnitModule},
    units::ScriptInstance,
};

#[derive(Debug, Asset, TypePath)]
pub struct ScriptModule {
    pub name: String,
    pub module: UnitModule,
}

impl ScriptModule {
    pub fn new(name: String, module: UnitModule) -> Self {
        Self { name, module }
    }

    pub fn instantiate(&self, settings: Option<SettingsValue>) -> ScriptInstance {
        ScriptInstance::new(self.module.instantiate(), settings)
    }
}
