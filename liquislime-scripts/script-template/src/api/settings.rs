use super::{FromWasmAbi, ToWasmAbi};

pub struct SettingsDescription(pub rmpv::Value);

impl ToWasmAbi for SettingsDescription {
    type Abi = <rmpv::Value as ToWasmAbi>::Abi;

    fn to_wasm_abi(&self) -> Self::Abi {
        self.0.to_wasm_abi()
    }
}

impl FromWasmAbi for SettingsDescription {
    type Abi = <rmpv::Value as FromWasmAbi>::Abi;

    fn from_wasm_abi(abi: Self::Abi) -> Self {
        Self(rmpv::Value::from_wasm_abi(abi))
    }
}

pub struct SettingsValue(pub rmpv::Value);

impl ToWasmAbi for SettingsValue {
    type Abi = <rmpv::Value as ToWasmAbi>::Abi;

    fn to_wasm_abi(&self) -> Self::Abi {
        self.0.to_wasm_abi()
    }
}

impl FromWasmAbi for SettingsValue {
    type Abi = <rmpv::Value as FromWasmAbi>::Abi;

    fn from_wasm_abi(abi: Self::Abi) -> Self {
        Self(rmpv::Value::from_wasm_abi(abi))
    }
}
