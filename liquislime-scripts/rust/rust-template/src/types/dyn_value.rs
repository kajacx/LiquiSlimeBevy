use crate::{
    api::{FromWasmAbi, ToWasmAbi},
    read_to_string, Deserialize, Serialize,
};
use anyhow::{bail, Result};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum DynValue {
    Null,
    Float64(f64),
    Text(String),
    Object(Vec<(String, DynValue)>),
}

impl DynValue {
    pub fn null() -> Self {
        Self::Null
    }

    pub fn float64(value: f64) -> Self {
        Self::Float64(value)
    }

    pub fn str(text: &str) -> Self {
        Self::string(text.to_owned())
    }

    pub fn string(text: String) -> Self {
        Self::Text(text)
    }

    pub fn object(pairs: impl IntoIterator<Item = (String, DynValue)>) -> Self {
        Self::Object(pairs.into_iter().collect())
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Self::Float64(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::Text(text) => Some(text),
            _ => None,
        }
    }

    pub fn try_into_string(self) -> Result<String, Self> {
        match self {
            Self::Text(text) => Ok(text),
            _ => Err(self),
        }
    }

    pub fn field(&self, field: &str) -> Option<&DynValue> {
        match self {
            Self::Object(values) => {
                values
                    .iter()
                    .find_map(|(key, value)| if key == field { Some(value) } else { None })
            }
            _ => None,
        }
    }
}

impl ToWasmAbi for DynValue {
    type Abi = <&'static [u8] as ToWasmAbi>::Abi;

    fn to_wasm_abi(&self) -> Self::Abi {
        let mut bytes = vec![];
        self.serialize(&mut bytes)
            .expect("writing to infinitely growable in-memory vec");
        bytes.as_slice().to_wasm_abi()
    }
}

impl FromWasmAbi for DynValue {
    type Abi = <Vec<u8> as FromWasmAbi>::Abi;

    fn from_wasm_abi(abi: Self::Abi) -> Result<Self> {
        let bytes = Vec::from_wasm_abi(abi)?;
        let mut cursor = std::io::Cursor::new(bytes);
        Self::deserialize(&mut cursor)
    }
}

impl Serialize for DynValue {
    fn serialize(&self, writer: &mut impl std::io::Write) -> Result<()> {
        match self {
            Self::Null => {
                rmp::encode::write_nil(writer)?;
            }
            Self::Float64(number) => {
                rmp::encode::write_f64(writer, *number)?;
            }
            Self::Text(text) => {
                rmp::encode::write_str(writer, text)?;
            }
            Self::Object(values) => {
                rmp::encode::write_map_len(writer, values.len() as u32)?;
                for (key, value) in values {
                    rmp::encode::write_str(writer, key)?;
                    value.serialize(writer)?;
                }
            }
        }
        Ok(())
    }
}

#[allow(deprecated)]
impl Deserialize for DynValue {
    fn deserialize(reader: &mut impl std::io::Read) -> Result<Self> {
        let marker = rmp::decode::read_marker(reader).expect("TODO: user error");
        Ok(match marker {
            rmp::Marker::Null => Self::Null,
            rmp::Marker::F64 => Self::Float64(rmp::decode::read_data_f64(reader)?),
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
                read_map(reader, len as usize)?
            }
            rmp::Marker::Map32 => {
                let len = rmp::decode::read_u32(reader)? as usize;
                read_map(reader, len as usize)?
            }
            _ => {
                bail!("Invalid marker in DynValue::deserialize: {marker:?}")
            }
        })
    }
}

fn read_map(reader: &mut impl std::io::Read, len: usize) -> Result<DynValue> {
    let mut values = Vec::with_capacity(len);

    for _ in 0..len {
        let name = String::deserialize(reader)?;
        let value = DynValue::deserialize(reader)?;
        values.push((name, value))
    }

    Ok(DynValue::Object(values))
}

impl Default for DynValue {
    fn default() -> Self {
        Self::Null
    }
}

impl From<f64> for DynValue {
    fn from(value: f64) -> Self {
        Self::Float64(value)
    }
}

impl From<String> for DynValue {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}

impl From<DynValue> for f64 {
    fn from(value: DynValue) -> Self {
        return value.as_f64().expect("TODO: user error");
    }
}

impl From<DynValue> for String {
    fn from(value: DynValue) -> Self {
        return value.try_into_string().expect("TODO: user error");
    }
}
