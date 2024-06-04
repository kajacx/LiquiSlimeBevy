use super::*;
use crate::api::{FromWasmAbi, ToWasmAbi};
use derive_more::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Faction(u8);

impl Faction {
    pub fn get_own_faction() -> Self {
        unsafe { Self::from_wasm_abi(crate::api::get_own_faction()) }
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

    fn from_wasm_abi(abi: Self::Abi) -> Self {
        Self(abi as u8)
    }
}
