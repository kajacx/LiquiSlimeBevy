use rmpv::Utf8String;

use super::*;

pub struct ScriptSettings {
    amount: SlimeAmount,
}

pub struct ScriptInstance {
    settings: ScriptSettings,
}

impl TryFrom<SettingsValue> for ScriptSettings {
    type Error = ();

    fn try_from(value: SettingsValue) -> Result<Self, Self::Error> {
        Ok(Self {
            amount: SlimeAmount::from_float(value.0.as_f64().unwrap()),
        })
    }
}

impl LiquislimeScript for ScriptInstance {
    type Settings = ScriptSettings;

    fn new_instance(settings: Self::Settings) -> Self {
        Self { settings }
    }

    fn describe_settings() -> rmpv::Value {
        rmpv::Value::String(Utf8String::from("SlimeAmount"))
    }

    fn change_settings(&mut self, settings: Self::Settings) {
        self.settings = settings
    }

    fn update(&mut self, time_elapsed: TimeInterval) {
        if let Some(position) = Mouse::is_pressed_in_bounds_at() {
            position
                .to_tile_position()
                .add_own_slime_amount(self.settings.amount * time_elapsed.to_seconds());
        }
    }
}
