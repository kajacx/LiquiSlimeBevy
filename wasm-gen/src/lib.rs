trait WasmRawType {
    fn get_type_name() -> &'static str;
}

impl WasmRawType for u64 {
    fn get_type_name() -> &'static str {
        "u64"
    }
}

impl WasmRawType for u32 {
    fn get_type_name() -> &'static str {
        "u32"
    }
}

// TODO: proper error handling
pub struct DeserializeError;

impl<T: core::fmt::Debug> From<T> for DeserializeError {
    fn from(value: T) -> Self {
        println!("Deserialization error: {:?}", value);
        Self
    }
}

mod carriers;
mod impls;
mod traits;

pub use traits::WasmSerializable;
