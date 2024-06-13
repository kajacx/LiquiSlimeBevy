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
            amount: value.0.into(),
        })
    }
}

impl LiquislimeScript for ScriptInstance {
    type Settings = ScriptSettings;

    fn new_instance(settings: Self::Settings) -> Self {
        Self { settings }
    }

    fn describe_settings() -> rmpv::Value {
        "SlimeAmount".into()
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
