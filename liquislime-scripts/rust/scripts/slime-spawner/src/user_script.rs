use super::*;

pub struct UserScript {
    settings: Settings,
}

pub struct Settings {
    amount: f64,
}

impl From<DynValue> for Settings {
    fn from(value: DynValue) -> Self {
        Self {
            amount: value.into(),
        }
    }
}

impl ScriptTemplate for UserScript {
    type Settings = Settings;

    fn describe_settings() -> SettingsDescription {
        SdFloat64 {
            default_value: 100.0,
        }
        .into()
    }

    fn new_instance(settings: Self::Settings) -> Self {
        Self { settings: settings }
    }

    fn change_settings(&mut self, settings: Self::Settings) {
        self.settings = settings.into()
    }

    fn update(&mut self, _time_elapsed: TimeInterval) {
        TilePosition::own_position().set_own_slime_amount_at_least(self.settings.amount);
    }
}
