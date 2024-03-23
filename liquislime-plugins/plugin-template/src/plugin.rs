use super::*;

pub struct Settings;

impl FromSettings for Settings {
    fn from_settings(_settings: protocol::SettingValues) -> Self {
        Self
    }
}

pub struct LiquislimeUnit(Settings);

impl LiquislimePlugin for LiquislimeUnit {
    type Settings = Settings;

    fn new(settings: Self::Settings) -> Self {
        Self(settings)
    }

    fn update(&mut self, time_elapsed: TimeInterval) {
        println!("Time elapsed: {:?}", time_elapsed);
    }
}
