use super::{
    helpers::{pack_u32s, unpack_u32s},
    FromWasmAbi, FromWasmAbiSimple, ScriptImpl, StoreData, ToWasmAbi, ToWasmAbiSimple, WasmAccess,
};
use anyhow::Result;
use wasmi::{Memory, StoreContextMut};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct FatPtr {
    addr: u32,
    len: u32,
}

impl ToWasmAbiSimple for FatPtr {
    type Abi = u64;

    fn to_wasm_abi_simple(&self) -> Self::Abi {
        pack_u32s(self.addr, self.len)
    }
}

impl FromWasmAbiSimple for FatPtr {
    type Abi = u64;

    fn from_wasm_abi_simple(abi: Self::Abi) -> Self {
        let unpacked = unpack_u32s(abi);

        Self {
            addr: unpacked.0,
            len: unpacked.1,
        }
    }
}

pub fn write_bytes(context: &mut WasmAccess, bytes: &[u8]) -> Result<FatPtr> {
    let fat_ptr = context
        .script()
        .with_exports(|exports| exports.allocate_bytes(context, bytes.len() as u32))?;

    context
        .script()
        .with_memory(|memory| memory.write(&mut context.store, fat_ptr.addr as usize, bytes))
        .expect("TODO: memory write");

    Ok(fat_ptr)
}

pub fn read_bytes(mut context: &mut WasmAccess, ptr: FatPtr) -> Result<Vec<u8>> {
    let mut bytes = vec![0; ptr.len as usize];

    context
        .script()
        .with_memory(|memory| memory.read(&mut context.store, ptr.addr as usize, &mut bytes))
        .expect("TODO: memory read");

    context
        .script()
        .with_exports(|exports| exports.free_bytes(context, ptr))?;

    Ok(bytes)
}
