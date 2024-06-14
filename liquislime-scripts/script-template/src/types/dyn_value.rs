use super::SlimeAmount;
use crate::{
    api::{FromWasmAbi, ToWasmAbi},
    read_to_string, Deserialize, Serialize,
};
use anyhow::{bail, Result};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum DynValue {
    Null,
    Number(f64),
    Text(String),
    SlimeAmount(SlimeAmount),
}

impl DynValue {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::Text(text) => Some(text),
            _ => None,
        }
    }

    pub fn into_string(self) -> Result<String, Self> {
        match self {
            Self::Text(text) => Ok(text),
            _ => Err(self),
        }
    }

    pub fn as_slime_amount(&self) -> Option<SlimeAmount> {
        match self {
            Self::SlimeAmount(amount) => Some(*amount),
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
}

impl Deserialize for DynValue {
    fn deserialize(reader: &mut impl std::io::Read) -> Result<Self> {
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
                    SlimeAmount::deserialize(reader)?.into()
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
