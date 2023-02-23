use crate::traits::WasmSerializable;

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

    fn deserialize_from(bytes: &[u8]) -> Result<Self, ()> {
        // TODO: the 4 bytes are (probably) copied here. Slightly annoying.
        let bytes: Result<[u8; 4], _> = bytes.try_into();
        match bytes {
            Ok(bytes) => Ok(Self::from_le_bytes(bytes)),
            Err(err) => {
                println!("TODO: proper error handling: {:?}", err);
                Err(())
            }
        }
    }
}
