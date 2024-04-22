use std::sync::Mutex;

wit_bindgen::generate!({
    path: "../../protocol.wit",
    world: "liquislime-unit",
    with: {
        "liquislime:protocol/types": types,
    }
});

#[allow(unused)]
mod types;
#[allow(unused)]
use types::*;

mod plugin;

static UNIT: Mutex<Option<plugin::LiquislimeUnit>> = Mutex::new(Option::None);

struct LiquislimeWorld;

impl Guest for LiquislimeWorld {
    fn describe_settings() -> String {
        r#"{
            "type": "Object",
            "value": [
                ["amount", {
                    "type": "SlimeAmount",
                    "value": {}
                }]
            ] 
        }"#
        .into()
    }

    fn init(settings: String) -> Result<(), String> {
        let settings: <plugin::LiquislimeUnit as LiquislimePlugin>::Settings =
            serde_json::from_str(&settings).unwrap(); // TODO: user error
        *UNIT.lock().unwrap() = Some(plugin::LiquislimeUnit::new(settings));
        Ok(())
    }

    fn change_settings(settings: String) -> Result<(), String> {
        let settings: <plugin::LiquislimeUnit as LiquislimePlugin>::Settings =
            serde_json::from_str(&settings).unwrap(); // TODO: user error
        UNIT.lock()
            .unwrap()
            .as_mut()
            .unwrap()
            .change_settings(settings);
        Ok(())
    }

    fn update(time_elapsed: TimeInterval) {
        UNIT.lock().unwrap().as_mut().unwrap().update(time_elapsed);
    }
}

trait LiquislimePlugin {
    type Settings: serde::de::DeserializeOwned;

    fn new(settings: Self::Settings) -> Self;

    fn change_settings(&mut self, settings: Self::Settings);

    fn update(&mut self, time_elapsed: TimeInterval);
}

export!(LiquislimeWorld);
