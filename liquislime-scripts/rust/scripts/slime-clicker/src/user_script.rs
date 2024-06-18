use super::*;
use settings::SdFloat64;

pub struct UserScript {
    settings: Settings,
}

pub struct Settings {
    amount: f64,
}

impl From<DynValue> for Settings {
    fn from(value: DynValue) -> Self {
        Self {
            amount: value.field("amount").unwrap().as_f64().unwrap(),
        }
    }
}

impl ScriptTemplate for UserScript {
    type Settings = Settings;

    fn describe_settings() -> SettingsDescription {
        SdObject(vec![(
            "amount".to_string(),
            SdFloat64 {
                default_value: 2000.0,
            }
            .into(),
        )])
        .into()
    }

    fn new_instance(settings: Self::Settings) -> Self {
        Self { settings }
    }

    fn change_settings(&mut self, settings: Self::Settings) {
        self.settings = settings;
    }

    fn update(&mut self, time_elapsed: TimeInterval) {
        if let Some(position) = Mouse::is_pressed_in_bounds_at() {
            position
                .to_tile_position()
                .add_own_slime_amount(self.settings.amount * time_elapsed.to_seconds());
        }
    }
}
