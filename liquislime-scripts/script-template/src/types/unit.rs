use crate::api::{FromWasmAbi, ToWasmAbi};
use anyhow::Result;
use derive_more::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Unit(u32);

impl Unit {
    pub fn get_current_unit() -> Self {
        Self::from_wasm_abi(unsafe { crate::api::get_current_unit() }).unwrap()
    }

    pub fn serialize(self, writer: &mut impl std::io::Write) -> Result<()> {
        Ok(rmp::encode::write_u32(writer, self.0)?)
    }

    pub fn deserialize(reader: &mut impl std::io::Read) -> Result<Self> {
        Ok(Self(rmp::decode::read_u32(reader)?))
    }
}

impl ToWasmAbi for Unit {
    type Abi = u32;

    fn to_wasm_abi(&self) -> Self::Abi {
        self.0
    }
}

impl FromWasmAbi for Unit {
    type Abi = u32;

    fn from_wasm_abi(abi: Self::Abi) -> Result<Self> {
        Ok(Self(abi))
    }
}
