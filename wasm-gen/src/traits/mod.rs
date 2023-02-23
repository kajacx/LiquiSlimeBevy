pub trait WasmSerializable: Sized {
    /** Total size of all data */
    fn static_size_bounds() -> (usize, Option<usize>);

    /** Flat size only */
    fn is_sized() -> bool;

    fn runtime_size(&self) -> usize;

    fn serialize_to<E>(&self, byte_sink: impl FnMut(&[u8]) -> Result<(), E>) -> Result<(), E>;

    fn deserialize_from(bytes: &[u8]) -> Result<Self, ()>;
}
