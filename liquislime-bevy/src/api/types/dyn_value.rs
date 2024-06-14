use super::{read_to_string, ApiSlimeAmount, Deserialize, Serialize};
use anyhow::{bail, Result};

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

    pub fn as_string_mut_anyway(&mut self, default: &str) -> &mut String {
        if !matches!(self, Self::Text(_)) {
            *self = Self::Text(String::from(default));
        };
        self.as_string_mut().unwrap()
    }

    pub fn as_slime_amount(&self) -> Option<ApiSlimeAmount> {
        match self {
            Self::SlimeAmount(amount) => Some(*amount),
            _ => None,
        }
    }

    pub fn serialize(&self, writer: &mut impl std::io::Write) -> Result<()> {
        match self {
            Self::Null => {
                rmp::encode::write_nil(writer)?;
            }
            Self::Number(number) => {
                rmp::encode::write_f64(writer, *number)?;
            }
            Self::Text(text) => {
                rmp::encode::write_str(writer, text)?;
            }
            Self::SlimeAmount(amount) => {
                rmp::encode::write_map_len(writer, 1)?;
                rmp::encode::write_str(writer, "SlimeAmount")?;
                amount.serialize(writer)?;
            }
        }
        Ok(())
    }

    pub fn deserialize(reader: &mut impl std::io::Read) -> Result<Self> {
        let marker = rmp::decode::read_marker(reader).expect("TODO: user error");
        Ok(match marker {
            rmp::Marker::Null => Self::Null,
            rmp::Marker::F64 => Self::Number(rmp::decode::read_f64(reader)?),
            rmp::Marker::FixStr(len) => read_to_string(reader, len as usize)?.into(),
            rmp::Marker::Str8 => {
                let len = rmp::decode::read_u8(reader)? as usize;
                read_to_string(reader, len)?.into()
            }
            rmp::Marker::Str16 => {
                let len = rmp::decode::read_u16(reader)? as usize;
                read_to_string(reader, len)?.into()
            }
            rmp::Marker::Str32 => {
                let len = rmp::decode::read_u32(reader)? as usize;
                read_to_string(reader, len)?.into()
            }
            rmp::Marker::FixMap(1) => {
                let tag = String::deserialize(reader)?;
                if tag == "SlimeAmount" {
                    ApiSlimeAmount::deserialize(reader)?.into()
                } else {
                    bail!("Invalid tag in DynValue::deserialize: {tag}")
                }
            }
            _ => {
                bail!("Invalid marker in DynValue::deserialize: {marker:?}")
            }
        })
    }
}

impl Default for DynValue {
    fn default() -> Self {
        Self::Null
    }
}

impl From<String> for DynValue {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}
