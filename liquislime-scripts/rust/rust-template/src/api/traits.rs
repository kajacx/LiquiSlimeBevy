use anyhow::Result;

use super::{read_bytes, write_bytes, FatPtr};

pub trait WasmAbi {}

impl WasmAbi for f32 {}
impl WasmAbi for f64 {}
impl WasmAbi for u32 {}
impl WasmAbi for u64 {}
impl WasmAbi for i32 {}
impl WasmAbi for i64 {}

pub trait ToWasmAbi {
    type Abi: WasmAbi;

    fn to_wasm_abi(&self) -> Self::Abi;
}

pub trait FromWasmAbi: Sized {
    type Abi: WasmAbi;

    fn from_wasm_abi(abi: Self::Abi) -> Result<Self>;
}

macro_rules! impl_primitive {
    ($name: ty) => {
        impl ToWasmAbi for $name {
            type Abi = Self;

            fn to_wasm_abi(&self) -> Self::Abi {
                *self
            }
        }

        impl FromWasmAbi for $name {
            type Abi = Self;

            fn from_wasm_abi(abi: Self::Abi) -> Result<Self> {
                Ok(abi)
            }
        }
    };
}

impl_primitive!(f32);
impl_primitive!(f64);
impl_primitive!(u32);
impl_primitive!(u64);
impl_primitive!(i32);
impl_primitive!(i64);

impl ToWasmAbi for bool {
    type Abi = u32;

    fn to_wasm_abi(&self) -> Self::Abi {
        match self {
            false => 0,
            true => 1,
        }
    }
}

impl FromWasmAbi for bool {
    type Abi = u32;

    fn from_wasm_abi(abi: Self::Abi) -> Result<Self> {
        Ok(!matches!(abi, 0))
    }
}

impl ToWasmAbi for &[u8] {
    type Abi = <FatPtr as ToWasmAbi>::Abi;

    fn to_wasm_abi(&self) -> Self::Abi {
        write_bytes(self).to_wasm_abi()
    }
}

impl FromWasmAbi for Vec<u8> {
    type Abi = <FatPtr as FromWasmAbi>::Abi;

    fn from_wasm_abi(abi: Self::Abi) -> Result<Self> {
        crate::log(format!(
            "About to read Vec<u8> {:?}",
            FatPtr::from_wasm_abi(abi)
        ));
        let res = read_bytes(FatPtr::from_wasm_abi(abi)?);
        crate::log(format!("Read Vec<u8> {:?}", res));
        Ok(res)
    }
}

impl ToWasmAbi for &str {
    type Abi = <&'static [u8] as ToWasmAbi>::Abi;

    fn to_wasm_abi(&self) -> Self::Abi {
        self.as_bytes().to_wasm_abi()
    }
}

impl FromWasmAbi for String {
    type Abi = <Vec<u8> as FromWasmAbi>::Abi;

    fn from_wasm_abi(abi: Self::Abi) -> Result<Self> {
        Ok(Self::from_utf8(Vec::<u8>::from_wasm_abi(abi)?)?)
    }
}
