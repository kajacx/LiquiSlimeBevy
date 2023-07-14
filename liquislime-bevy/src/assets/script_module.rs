use bevy::reflect::{TypePath, TypeUuid};

use crate::{api::UnitModule, units::ScriptInstance};

#[derive(Debug, TypeUuid, TypePath)]
#[uuid = "39f0d1f8-a7eb-4eaa-887b-4f31a73c196e"]
pub struct ScriptModule {
    pub name: String,
    pub module: UnitModule,
}

impl ScriptModule {
    pub fn new(name: String, module: UnitModule) -> Self {
        Self { name, module }
    }

    pub fn instantiate(&self) -> ScriptInstance {
        ScriptInstance::new(self.module.instantiate())
    }
}
