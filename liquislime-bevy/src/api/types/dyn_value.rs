use super::ApiSlimeAmount;
use wasm_bridge::Result;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum DynValue {
    Null,
    Number(f64),
    Text(String),
    SlimeAmount(ApiSlimeAmount),
}

impl DynValue {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::Text(text) => Some(text),
            _ => None,
        }
    }

    pub fn as_string_mut(&mut self) -> Option<&mut String> {
        match self {
            Self::Text(text) => Some(text),
            _ => None,
        }
    }

    pub fn as_slime_amount(&self) -> Option<ApiSlimeAmount> {
        match self {
            Self::SlimeAmount(amount) => Some(*amount),
            _ => None,
        }
    }

    pub fn serialize(&self) -> rmpv::Value {
        match self {
            // TODO: isn't self-describing ...
            Self::SlimeAmount(amount) => amount.0.into(),
            _ => todo!(),
        }
    }

    pub fn deserialize(value: rmpv::Value) -> Result<Self> {
        Ok(Self::SlimeAmount(ApiSlimeAmount(
            value.as_i64().expect("TODO: rework completely"),
        )))
    }
}

impl From<ApiSlimeAmount> for DynValue {
    fn from(value: ApiSlimeAmount) -> Self {
        Self::SlimeAmount(value)
    }
}

impl From<String> for DynValue {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}
