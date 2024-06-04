use rmpv::Utf8String;

use super::*;

pub struct ScriptSettings {
    name: String,
}

pub struct ScriptInstance {
    settings: ScriptSettings,
}

impl TryFrom<SettingsValue> for ScriptSettings {
    type Error = ();

    fn try_from(value: SettingsValue) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.0.to_string(),
        })
    }
}

impl LiquislimeScript for ScriptInstance {
    type Settings = ScriptSettings;

    fn describe_settings() -> rmpv::Value {
        rmpv::Value::String(Utf8String::from("String"))
    }

    fn new_instance(settings: Self::Settings) -> Self {
        Self { settings }
    }

    fn change_settings(&mut self, settings: Self::Settings) {
        self.settings = settings
    }

    fn update(&mut self, time_elapsed: TimeInterval) {
        println!("[{}] Time elapsed: {:?}", self.settings.name, time_elapsed);
    }
}
