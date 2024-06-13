// use super::{FromWasmAbi, ToWasmAbi};
// use crate::DynValue;
// use anyhow::Result;

// pub struct SettingsDescription(pub DynValue);

// impl ToWasmAbi for SettingsDescription {
//     type Abi = <DynValue as ToWasmAbi>::Abi;

//     fn to_wasm_abi(&self) -> Self::Abi {
//         self.0.to_wasm_abi()
//     }
// }

// impl FromWasmAbi for SettingsDescription {
//     type Abi = <DynValue as FromWasmAbi>::Abi;

//     fn from_wasm_abi(abi: Self::Abi) -> Result<Self> {
//         Ok(Self(DynValue::from_wasm_abi(abi)?))
//     }
// }

// pub struct SettingsValue(pub DynValue);

// impl ToWasmAbi for SettingsValue {
//     type Abi = <DynValue as ToWasmAbi>::Abi;

//     fn to_wasm_abi(&self) -> Self::Abi {
//         self.0.to_wasm_abi()
//     }
// }

// impl FromWasmAbi for SettingsValue {
//     type Abi = <DynValue as FromWasmAbi>::Abi;

//     fn from_wasm_abi(abi: Self::Abi) -> Result<Self> {
//         Ok(Self(DynValue::from_wasm_abi(abi)?))
//     }
// }
