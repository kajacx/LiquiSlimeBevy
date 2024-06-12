use super::{Script, ScriptInstance, INSTANCES};
use crate::api::SettingsValue;
use wasm_bridge::Result;

#[derive(Debug)]
pub struct ScriptRequest {
    id: u32,
    script: Script,
    settings: SettingsValue,
}

impl ScriptRequest {
    pub fn new(script: Script, settings: SettingsValue) -> Self {
        let id = INSTANCES.try_lock().unwrap().insert(()) as u32;

        Self {
            id,
            script,
            settings,
        }
    }

    pub fn try_initialize(&self) -> Result<Option<ScriptInstance>> {
        self.script
            .with_loaded(|loaded| {
                // TODO: Settings clone
                loaded
                    .new_instance_with_id(self.id, self.settings.clone())
                    .map(Option::Some)
            })
            .unwrap_or(Ok(None))
    }
}
