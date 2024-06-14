use super::*;
use anyhow::bail;
use settings::SdText;

pub struct Settings {
    name: String,
}

pub struct UserScript {
    settings: Settings,
}

impl SettingsTemplate for Settings {
    fn describe_settings() -> SettingsDescription {
        SettingsDescription::Text(SdText)
    }

    fn default_value() -> SettingsValue {
        SettingsValue("My first script".to_owned().into())
    }

    fn parse(value: SettingsValue) -> Result<Self> {
        match value.0.into_string() {
            Ok(text) => Ok(Self { name: text.into() }),
            Err(value) => bail!("Expected string, got {value:?}"),
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
        log(format!(
            "[{}] Time elapsed: {:?}",
            self.settings.name, time_elapsed
        ));
    }
}
