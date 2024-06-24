use super::*;

pub struct UserScript {
    settings: Settings,
}

pub struct Settings {
    name: String,
}

impl From<DynValue> for Settings {
    fn from(value: DynValue) -> Self {
        Self {
            name: value
                .field("name")
                .expect("TODO: user error")
                .clone()
                .into(),
        }
    }
}

impl ScriptTemplate for UserScript {
    type Settings = Settings;

    fn describe_settings() -> SettingsDescription {
        SdObject(vec![(
            "name".to_string(),
            SdString {
                default_value: "Hello script".to_string(),
            }
            .into(),
        )])
        .into()
    }

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
