use crate::api::{FromWasmAbi, ToWasmAbi};
use anyhow::Result;
use derive_more::{Add, AddAssign, Sub, SubAssign};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Faction(u8);

impl Faction {
    pub fn get_own_faction() -> Self {
        Self::from_wasm_abi(unsafe { crate::api::get_own_faction() }).unwrap()
    }

    pub fn serialize(self, writer: &mut impl std::io::Write) -> Result<()> {
        Ok(rmp::encode::write_u8(writer, self.0)?)
    }

    pub fn deserialize(reader: &mut impl std::io::Read) -> Result<Self> {
        Ok(Self(rmp::decode::read_u8(reader)?))
    }
}

impl ToWasmAbi for Faction {
    type Abi = u32;

    fn to_wasm_abi(&self) -> Self::Abi {
        self.0 as u32
    }
}

impl FromWasmAbi for Faction {
    type Abi = u32;

    fn from_wasm_abi(abi: Self::Abi) -> Result<Self> {
        Ok(Self(abi as u8))
    }
}
