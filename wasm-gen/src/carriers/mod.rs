trait Carrier: Copy {
    type SerializeError;

    fn serialize<T: WasmSerializable>(value: &T) -> Result<Self, SerializeError>;

    fn deserialize<T: WasmSerializable>(Self) -> Result<T, DeserializeError>;
}

mod u64_const;
mod u64_fatptr;
mod u64_mixed;

pub use u64_const::*;
pub use u64_fatptr::*;
pub use u64_mixed::*;
