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

    fn describe_settings() -> rmpv::Value {
        "SlimeAmount".into()
    }

    fn new_instance(settings: Self::Settings) -> Self {
        Self { settings }
    }

    fn change_settings(&mut self, settings: Self::Settings) {
        self.settings = settings
    }

    fn update(&mut self, _time_elapsed: TimeInterval) {
        TilePosition::own_position().set_own_slime_amount_at_least(self.settings.amount);
    }
}
