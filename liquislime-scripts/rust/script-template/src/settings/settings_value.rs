use crate::{
    api::{FromWasmAbi, ToWasmAbi},
    DynValue,
};
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct SettingsValue(pub DynValue);

impl ToWasmAbi for SettingsValue {
    type Abi = <DynValue as ToWasmAbi>::Abi;

    fn to_wasm_abi(&self) -> Self::Abi {
        self.0.to_wasm_abi()
    }
}

impl FromWasmAbi for SettingsValue {
    type Abi = <DynValue as ToWasmAbi>::Abi;

    fn from_wasm_abi(abi: Self::Abi) -> Result<Self> {
        Ok(Self(DynValue::from_wasm_abi(abi)?))
    }
}
