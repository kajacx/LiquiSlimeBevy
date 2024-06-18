use crate::api::ToWasmAbi;
use crate::DynValue;
use derive_more::From;

mod float64;
mod none;
mod object;
mod string;

#[allow(unused)]
pub use float64::*;
#[allow(unused)]
pub use none::*;
#[allow(unused)]
pub use object::*;
#[allow(unused)]
pub use string::*;

#[derive(Debug, Clone, From)]
pub enum SettingsDescription {
    // None(#[from] SdNone),
    // Float64(#[from] SdFloat64),
    // String(#[from] SdString),
    // Object(#[from] SdObject),
    None(SdNone),
    Float64(SdFloat64),
    String(SdString),
    Object(SdObject),
}

impl SettingsDescription {
    pub fn describe(&self) -> DynValue {
        match self {
            Self::None(none) => none.describe(),
            Self::Float64(value) => value.describe(),
            Self::String(string) => string.describe(),
            Self::Object(object) => object.describe(),
        }
    }
}

impl ToWasmAbi for SettingsDescription {
    type Abi = <DynValue as ToWasmAbi>::Abi;

    fn to_wasm_abi(&self) -> Self::Abi {
        self.describe().to_wasm_abi()
    }
}
