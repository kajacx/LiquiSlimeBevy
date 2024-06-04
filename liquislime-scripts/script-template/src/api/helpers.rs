use super::{FromWasmAbi, ToWasmAbi};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct FatPtr {
    addr: u32,
    len: u32,
}

impl ToWasmAbi for FatPtr {
    type Abi = u64;

    fn to_wasm_abi(&self) -> Self::Abi {
        pack_u32s(self.addr, self.len)
    }
}

impl FromWasmAbi for FatPtr {
    type Abi = u64;

    fn from_wasm_abi(abi: Self::Abi) -> Self {
        let unpacked = unpack_u32s(abi);

        Self {
            addr: unpacked.0,
            len: unpacked.1,
        }
    }
}

pub fn write_bytes(bytes: &[u8]) -> FatPtr {
    // TODO: forced copy, even when caller doesn't need to keep the data anyway
    let data = bytes.to_owned().leak();

    FatPtr {
        addr: data.as_ptr() as usize as u32,
        len: data.len() as u32,
    }
}

pub fn read_bytes(ptr: FatPtr) -> Vec<u8> {
    // SAFETY: We trust the host allocates memory properly
    unsafe {
        Vec::from_raw_parts(
            ptr.addr as usize as *mut u8,
            ptr.len as usize,
            ptr.len as usize,
        )
    }
}

pub fn allocate_bytes_impl(len: u32) -> FatPtr {
    let data = vec![0u8; len as usize].leak();

    FatPtr {
        addr: data.as_ptr() as usize as u32,
        len: data.len() as u32,
    }
}

pub fn free_bytes_impl(ptr: FatPtr) {
    // SAFETY: we trust the host to call this correctly
    unsafe {
        drop(Vec::from_raw_parts(
            ptr.addr as usize as *mut u8,
            ptr.len as usize,
            ptr.len as usize,
        ));
    }
}

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

#[cfg(test)]
mod tests {
    pub use super::*;

    #[test]
    fn test_roundtrip_fat_ptr() {
        let fat_ptr = FatPtr { addr: 30, len: 15 };

        let as_abi = fat_ptr.to_wasm_abi();

        let as_fat_ptr = FatPtr::from_wasm_abi(as_abi);

        assert_eq!(fat_ptr, as_fat_ptr);
    }

    #[test]
    fn test_roundtrip_f32s() {
        let values = (3.5f32, -4.25f32);

        let packed = pack_f32s(values.0, values.1);

        let unpacked = unpack_f32s(packed);

        assert_eq!(values, unpacked);
    }
}
