use super::{Script, ScriptInstance};
use crate::api::SettingsValue;
use wasm_bridge::Result;

#[derive(Debug)]
pub struct ScriptRequest {
    script: Script,
    settings: SettingsValue,
}

impl ScriptRequest {
    pub fn new(script: Script, settings: SettingsValue) -> Self {
        Self { script, settings }
    }

    pub fn try_initialize(&self) -> Result<Option<ScriptInstance>> {
        self.script
            .with_loaded(|loaded| {
                // TODO: Script and Settings clone
                loaded
                    .new_instance(self.script.clone(), self.settings.clone())
                    .map(Option::Some)
            })
            .unwrap_or(Ok(None))
    }
}
