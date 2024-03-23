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
    fn init(settings: protocol::SettingValues) {
        *UNIT.lock().unwrap() = Some(plugin::LiquislimeUnit::new(
            <plugin::LiquislimeUnit as LiquislimePlugin>::Settings::from_settings(settings),
        ))
    }

    fn update(time_elapsed: protocol::TimeInterval) {
        plugin::LiquislimeUnit::update(
            UNIT.lock().unwrap().as_mut().unwrap(),
            TimeInterval::from_protocol(time_elapsed),
        );
    }
}

trait FromSettings {
    fn from_settings(settings: protocol::SettingValues) -> Self;
}

trait LiquislimePlugin {
    type Settings: FromSettings;

    fn new(settings: Self::Settings) -> Self;

    fn update(&mut self, time_elapsed: TimeInterval);
}
