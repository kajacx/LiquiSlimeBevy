use crate::DynValue;

#[derive(Debug, Clone, Default)]
pub struct SdString {
    pub default_value: String,
}

impl SdString {
    pub fn describe(&self) -> DynValue {
        DynValue::object([
            ("type".to_string(), DynValue::str("String")),
            (
                "default_value".to_string(),
                DynValue::string(self.default_value.clone()),
            ),
        ])
    }
}
