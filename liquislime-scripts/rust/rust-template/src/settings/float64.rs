use crate::DynValue;

#[derive(Debug, Clone, Default)]
pub struct SdFloat64 {
    pub default_value: f64,
}

impl SdFloat64 {
    pub fn describe(&self) -> DynValue {
        DynValue::object([
            ("type".to_string(), DynValue::str("Float64")),
            (
                "default_value".to_string(),
                DynValue::float64(self.default_value),
            ),
        ])
    }
}
