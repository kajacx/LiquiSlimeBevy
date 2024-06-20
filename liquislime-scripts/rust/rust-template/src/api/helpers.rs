pub fn pack_u32s(num1: u32, num2: u32) -> u64 {
    (num1 as u64) | ((num2 as u64) << 32)
}

pub fn unpack_u32s(packed: u64) -> (u32, u32) {
    let addr = packed as u32;
    let len = (packed >> 32) as u32;

    (addr, len)
}

pub fn pack_f32s(num1: f32, num2: f32) -> u64 {
    pack_u32s(
        u32::from_le_bytes(num1.to_le_bytes()),
        u32::from_le_bytes(num2.to_le_bytes()),
    )
}

pub fn unpack_f32s(packed: u64) -> (f32, f32) {
    let unpacked = unpack_u32s(packed);
    (
        f32::from_le_bytes(unpacked.0.to_le_bytes()),
        f32::from_le_bytes(unpacked.1.to_le_bytes()),
    )
}

pub fn debug_print_fat_ptr(ptr: super::FatPtr) {
    let bytes = super::read_bytes(ptr);
    let mut text = "[".to_string();

    let mut first = true;
    for byte in &bytes {
        if first {
            first = false;
        } else {
            text.push(',');
            text.push(' ');
        }
        text.push_str(&format!("{byte:#03} {byte:#010b}"));
    }
    text.push(']');

    crate::log(text);

    std::mem::forget(bytes);
}

#[cfg(test)]
mod tests {
    pub use super::*;

    #[test]
    fn test_roundtrip_f32s() {
        let values = (3.5f32, -4.25f32);

        let packed = pack_f32s(values.0, values.1);

        let unpacked = unpack_f32s(packed);

        assert_eq!(values, unpacked);
    }
}
