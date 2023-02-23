use crate::traits::WasmSerializable;

impl WasmSerializable for String {
    fn static_size_bounds() -> (usize, Option<usize>) {
        (0, None)
    }

    fn is_sized() -> bool {
        false
    }

    fn runtime_size(&self) -> usize {
        self.len()
    }

    fn serialize_to<E>(&self, mut byte_sink: impl FnMut(&[u8]) -> Result<(), E>) -> Result<(), E> {
        byte_sink(&self.as_bytes())
    }

    fn deserialize_from(bytes: &[u8]) -> Result<Self, ()> {
        let string = String::from_utf8(Vec::from(bytes));
        match string {
            Ok(string) => Ok(string),
            Err(err) => {
                println!("TODO: proper error handling: {:?}", err);
                Err(())
            }
        }
    }
}
