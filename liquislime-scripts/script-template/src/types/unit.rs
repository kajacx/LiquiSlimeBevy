use super::*;
use crate::api::{FromWasmAbi, ToWasmAbi};
use derive_more::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Unit(u32);

impl Unit {
    pub fn get_current_unit() -> Self {
        unsafe { Self::from_wasm_abi(crate::api::get_current_unit()) }
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

    fn from_wasm_abi(abi: Self::Abi) -> Self {
        Self(abi)
    }
}
