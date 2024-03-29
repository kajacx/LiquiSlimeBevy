use super::*;

#[derive(serde::Deserialize)]
pub struct UnitSettings {
    name: String,
}

pub struct LiquislimeUnit {
    settings: UnitSettings,
}

impl LiquislimePlugin for LiquislimeUnit {
    type Settings = UnitSettings;

    fn new(settings: Self::Settings) -> Self {
        Self { settings }
    }

    fn change_settings(&mut self, settings: Self::Settings) {
        self.settings = settings
    }

    fn update(&mut self, time_elapsed: TimeInterval) {
        println!("[{}] Time elapsed: {:?}", self.settings.name, time_elapsed);
    }
}
