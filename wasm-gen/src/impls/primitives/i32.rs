use crate::traits::WasmSerializable;
use crate::DeserializeError;

impl WasmSerializable for i32 {
    fn static_size_bounds() -> (usize, Option<usize>) {
        (4, Some(4))
    }

    fn is_sized() -> bool {
        true
    }

    fn runtime_size(&self) -> usize {
        4
    }

    fn serialize_to<E>(&self, mut byte_sink: impl FnMut(&[u8]) -> Result<(), E>) -> Result<(), E> {
        byte_sink(&self.to_le_bytes())
    }

    fn deserialize_from(bytes: &[u8]) -> Result<Self, DeserializeError> {
        // TODO: the 4 bytes are (probably) copied here. Slightly annoying.
        Ok(Self::from_le_bytes(bytes.try_into()?))
    }
}
