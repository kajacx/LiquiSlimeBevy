use super::*;
use crate::api::{FromWasmAbi, ToWasmAbi};
use derive_more::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Instance(u32);

impl Instance {
    pub fn get_current_instance() -> Self {
        unsafe { Self::from_wasm_abi(crate::api::get_current_instance()) }
    }
}

impl ToWasmAbi for Instance {
    type Abi = u32;

    fn to_wasm_abi(&self) -> Self::Abi {
        self.0
    }
}

impl FromWasmAbi for Instance {
    type Abi = u32;

    fn from_wasm_abi(abi: Self::Abi) -> Self {
        Self(abi)
    }
}
