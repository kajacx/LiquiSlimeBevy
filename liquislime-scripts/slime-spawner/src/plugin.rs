use super::*;
use anyhow::bail;
use settings::SdSlimeAmount;

pub struct Settings {
    amount: SlimeAmount,
}

pub struct UserScript {
    settings: Settings,
}

impl SettingsTemplate for Settings {
    fn describe_settings() -> SettingsDescription {
        SettingsDescription::SlimeAmount(SdSlimeAmount)
    }

    fn default_value() -> SettingsValue {
        SettingsValue(SlimeAmount::from_integer(100).into())
    }

    fn parse(value: SettingsValue) -> Result<Self> {
        match value.0.as_slime_amount() {
            Some(amount) => Ok(Self { amount }),
            None => bail!("Expected SlimeAmount, got {value:?}"),
        }
    }
}

impl ScriptTemplate for UserScript {
    type Settings = Settings;

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
