use crate::DynValue;

#[derive(Debug, Clone, Default)]
pub struct SdNone;

impl SdNone {
    pub fn describe(&self) -> DynValue {
        DynValue::object([("type".to_string(), DynValue::str("None"))])
    }

    pub fn default_value(&self) -> DynValue {
        DynValue::null()
    }
}
