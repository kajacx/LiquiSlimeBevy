use super::Carrier;
use crate::*;

struct U64ConstCarrier;

struct TooManyBytesError;

impl Carrier for U64ConstCarrier {
    type CarrierType = u64;
    type SerializeError = TooManyBytesError;

    fn serialize<T: WasmSerializable>(
        value: &T,
    ) -> Result<Self::CarrierType, Self::SerializeError> {
        let mut bytes = [0u8; 8];
        let mut index = 0;

        let byte_sink = |new_bytes: &[u8]| {
            let new_index = index + new_bytes.len();
            if new_index > bytes.len() {
                Err(TooManyBytesError)
            } else {
                bytes[index..new_index].clone_from_slice(new_bytes);
                index = new_index;
                Ok(())
            }
        };

        value.serialize_to(byte_sink)?;

        Ok(u64::from_le_bytes(bytes))
    }

    fn deserialize<T: WasmSerializable>(value: Self::CarrierType) -> Result<T, DeserializeError> {
        let bytes = value.to_le_bytes();

        // TODO: we are relying on T not reading the slice length for useful information
        T::deserialize_from(&bytes)
    }
}
