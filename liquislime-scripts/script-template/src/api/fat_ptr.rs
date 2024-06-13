use super::{pack_u32s, unpack_u32s, FromWasmAbi, ToWasmAbi};
use anyhow::Result;

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

    fn from_wasm_abi(abi: Self::Abi) -> Result<Self> {
        let unpacked = unpack_u32s(abi);

        Ok(Self {
            addr: unpacked.0,
            len: unpacked.1,
        })
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

#[cfg(test)]
mod tests {
    pub use super::*;

    #[test]
    fn test_roundtrip_fat_ptr() {
        let fat_ptr = FatPtr { addr: 30, len: 15 };

        let as_abi = fat_ptr.to_wasm_abi();

        let as_fat_ptr = FatPtr::from_wasm_abi(as_abi).unwrap();

        assert_eq!(fat_ptr, as_fat_ptr);
    }
}
