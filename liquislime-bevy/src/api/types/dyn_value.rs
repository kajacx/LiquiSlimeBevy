use super::{read_to_string, ApiSlimeAmount, Deserialize, Serialize};
use anyhow::{bail, Result};
use rmp::decode::RmpRead;
use std::error::Error;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum DynValue {
    Null,
    Float64(f64),
    String(String),
    Object(Vec<(String, DynValue)>),
}

impl DynValue {
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Self::Float64(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(text) => Some(text),
            _ => None,
        }
    }

    pub fn as_string_mut(&mut self) -> Option<&mut String> {
        match self {
            Self::String(text) => Some(text),
            _ => None,
        }
    }

    pub fn as_string_mut_anyway(&mut self, default: &str) -> &mut String {
        if !matches!(self, Self::String(_)) {
            *self = Self::String(String::from(default));
        };
        self.as_string_mut().unwrap()
    }

    pub fn field(&self, field: &str) -> Result<&DynValue, FieldError> {
        match self {
            Self::Object(values) => values
                .iter()
                .find_map(|(key, value)| if key == field { Some(value) } else { None })
                .ok_or_else(|| FieldError::FieldNotFound(format!("{self:?}"), field.to_string())),
            _ => Err(FieldError::NotAnObject(format!("{self:?}"))),
        }
    }

    pub fn field_mut<'a>(&'a mut self, field: &str) -> Result<&'a mut DynValue, FieldError> {
        if let Self::Object(values) = self {
            for (key, value) in values {
                if key == field {
                    // SAFETY: Rust is too stupid to understand that the value isn't used in the Err branches
                    return Ok(unsafe { &mut *(value as *mut _) });
                }
            }
            return Err(FieldError::FieldNotFound(
                format!("{self:?}"),
                field.to_string(),
            ));
        } else {
            return Err(FieldError::NotAnObject(format!("{self:?}")));
        }
    }

    pub fn fields(&self) -> Option<impl Iterator<Item = (&str, &DynValue)> + '_> {
        match self {
            Self::Object(values) => Some(values.iter().map(|(key, value)| (key.as_str(), value))),
            _ => None,
        }
    }

    pub fn serialize(&self, writer: &mut impl std::io::Write) -> Result<()> {
        match self {
            Self::Null => {
                rmp::encode::write_nil(writer)?;
            }
            Self::Float64(value) => {
                rmp::encode::write_f64(writer, *value)?;
            }
            Self::String(text) => {
                rmp::encode::write_str(writer, text)?;
            }
            Self::Object(fields) => {
                rmp::encode::write_map_len(writer, fields.len() as u32)?;
                for (key, value) in fields.iter() {
                    rmp::encode::write_str(writer, &key)?;
                    value.serialize(writer)?;
                }
            }
        }
        Ok(())
    }

    #[allow(deprecated)]
    pub fn deserialize(reader: &mut impl std::io::Read) -> Result<Self> {
        let marker = rmp::decode::read_marker(reader).expect("TODO: user error");
        println!("read marker: {marker:?} {}", marker.to_u8());
        Ok(match marker {
            rmp::Marker::Null => Self::Null,
            rmp::Marker::F64 => Self::Float64(reader.read_data_f64()?),
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
            rmp::Marker::FixMap(len) => read_map(reader, len as usize)?,
            rmp::Marker::Map16 => {
                let len = rmp::decode::read_u16(reader)? as usize;
                read_map(reader, len)?
            }
            rmp::Marker::Map32 => {
                let len = rmp::decode::read_u32(reader)? as usize;
                read_map(reader, len)?
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
        Self::String(value)
    }
}

fn read_map(reader: &mut impl std::io::Read, length: usize) -> Result<DynValue> {
    let mut vec = Vec::with_capacity(length);
    for _ in 0..length {
        let str_length = rmp::decode::read_str_len(reader)?;
        let key = read_to_string(reader, str_length as usize)?;
        let value = DynValue::deserialize(reader)?;
        vec.push((key, value));
    }
    Ok(DynValue::Object(vec))
}

#[derive(Debug, Clone)]
pub enum FieldError {
    NotAnObject(String),
    FieldNotFound(String, String),
}

impl Error for FieldError {}

impl std::fmt::Display for FieldError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotAnObject(value) => writeln!(f, "DynValue '{value:?}' is not an object"),
            Self::FieldNotFound(value, field) => {
                writeln!(f, "Field '{field}' does not exist on {value:?}")
            }
        }
    }
}
