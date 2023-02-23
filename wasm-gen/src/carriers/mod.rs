use crate::*;

trait Carrier: Copy {
    type CarrierType: Copy;
    type SerializeError;

    fn serialize<T: WasmSerializable>(value: &T)
        -> Result<Self::CarrierType, Self::SerializeError>;

    fn deserialize<T: WasmSerializable>(value: Self::CarrierType) -> Result<T, DeserializeError>;
}

mod u64_const;
mod u64_fatptr;
mod u64_mixed;

pub use u64_const::*;
pub use u64_fatptr::*;
pub use u64_mixed::*;
