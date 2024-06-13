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
        SettingsValue("My first script".to_owned().into())
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

    fn update(&mut self, time_elapsed: TimeInterval) {
        if let Some(position) = Mouse::is_pressed_in_bounds_at() {
            position
                .to_tile_position()
                .add_own_slime_amount(self.settings.amount * time_elapsed.to_seconds());
        }
    }
}
