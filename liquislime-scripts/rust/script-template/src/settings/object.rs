use super::SettingsDescription;
use crate::DynValue;

#[derive(Debug, Clone, Default)]
pub struct SdObject(pub Vec<(String, SettingsDescription)>);

impl SdObject {
    pub fn describe(&self) -> DynValue {
        DynValue::object([
            ("type".to_string(), DynValue::str("Object")),
            (
                "values".to_string(),
                DynValue::object(
                    self.0
                        .iter()
                        .cloned()
                        .map(|(key, value)| (key, value.describe())),
                ),
            ),
        ])
    }
}
