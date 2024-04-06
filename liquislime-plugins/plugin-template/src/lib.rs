mod protocol {
    use super::*;

    wit_bindgen::generate!({
        path: "../../protocol.wit",
        world: "liquislime-unit",
        exports: {
           world: LiquislimeWorld
        }
    });
}

mod plugin;
#[allow(unused)]
mod types;

mod settings;

use std::sync::Mutex;

use types::*;

static UNIT: Mutex<Option<plugin::LiquislimeUnit>> = Mutex::new(Option::None);

struct LiquislimeWorld;

impl protocol::Guest for LiquislimeWorld {
    fn describe_settings() -> String {
        "TODO: describe settings ".into()
    }

    fn default_settings() -> String {
        "TODO: default settings ".into()
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

    fn update(time_elapsed: protocol::TimeInterval) {
        UNIT.lock()
            .unwrap()
            .as_mut()
            .unwrap()
            .update(TimeInterval::from_protocol(time_elapsed));
    }
}

trait LiquislimePlugin {
    type Settings: serde::de::DeserializeOwned;

    fn new(settings: Self::Settings) -> Self;

    fn change_settings(&mut self, settings: Self::Settings);

    fn update(&mut self, time_elapsed: TimeInterval);
}
