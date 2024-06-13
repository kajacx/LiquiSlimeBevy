use super::{
    memory_manage::{read_bytes, write_bytes, FatPtr},
    script_impl, ScriptImpl, StoreData,
};
use anyhow::bail;
use wasm_bridge::{AsContextMut, Result, StoreContextMut};

pub trait WasmAbi {}

impl WasmAbi for f32 {}
impl WasmAbi for f64 {}
impl WasmAbi for u32 {}
impl WasmAbi for u64 {}
impl WasmAbi for i32 {}
impl WasmAbi for i64 {}

pub struct WasmAccess<'a> {
    pub store: StoreContextMut<'a, StoreData>,
    // pub script: ScriptImpl,
}

impl<'a> WasmAccess<'a> {
    pub fn script(&self) -> ScriptImpl {
        self.store.data().current_script
    }
}

pub trait ToWasmAbi {
    type Abi: WasmAbi;

    fn to_wasm_abi(&self, context: &mut WasmAccess) -> Result<Self::Abi>;
}

pub trait ToWasmAbiSimple {
    type Abi: WasmAbi;

    fn to_wasm_abi_simple(&self) -> Self::Abi;
}

impl<T: ToWasmAbiSimple> ToWasmAbi for T {
    type Abi = <T as ToWasmAbiSimple>::Abi;

    fn to_wasm_abi(&self, _: &mut WasmAccess) -> Result<Self::Abi> {
        Ok(self.to_wasm_abi_simple())
    }
}

pub trait FromWasmAbi: Sized {
    type Abi: WasmAbi;

    fn from_wasm_abi(context: &mut WasmAccess, abi: Self::Abi) -> Result<Self>;
}

pub trait FromWasmAbiSimple: Sized {
    type Abi: WasmAbi;

    fn from_wasm_abi_simple(abi: Self::Abi) -> Self;
}

impl<T: FromWasmAbiSimple> FromWasmAbi for T {
    type Abi = <T as FromWasmAbiSimple>::Abi;

    fn from_wasm_abi(_: &mut WasmAccess, abi: Self::Abi) -> Result<Self> {
        Ok(T::from_wasm_abi_simple(abi))
    }
}

macro_rules! impl_primitive {
    ($name: ty) => {
        impl ToWasmAbiSimple for $name {
            type Abi = Self;

            fn to_wasm_abi_simple(&self) -> Self::Abi {
                *self
            }
        }

        impl FromWasmAbiSimple for $name {
            type Abi = Self;

            fn from_wasm_abi_simple(abi: Self::Abi) -> Self {
                abi
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

impl ToWasmAbiSimple for bool {
    type Abi = u32;

    fn to_wasm_abi_simple(&self) -> Self::Abi {
        match self {
            false => 0,
            true => 1,
        }
    }
}

impl FromWasmAbiSimple for bool {
    type Abi = u32;

    fn from_wasm_abi_simple(abi: Self::Abi) -> Self {
        !matches!(abi, 0)
    }
}

impl ToWasmAbi for &[u8] {
    type Abi = <FatPtr as ToWasmAbi>::Abi;

    fn to_wasm_abi(&self, context: &mut WasmAccess) -> Result<Self::Abi> {
        let ptr = write_bytes(context, self)?;
        let abi = ptr.to_wasm_abi_simple();
        Ok(abi)
    }
}

impl FromWasmAbi for Vec<u8> {
    type Abi = <FatPtr as FromWasmAbi>::Abi;

    fn from_wasm_abi(context: &mut WasmAccess, abi: Self::Abi) -> Result<Self> {
        read_bytes(context, FatPtr::from_wasm_abi_simple(abi))
    }
}

impl ToWasmAbi for &str {
    type Abi = <&'static [u8] as ToWasmAbi>::Abi;

    fn to_wasm_abi(&self, context: &mut WasmAccess) -> Result<Self::Abi> {
        self.as_bytes().to_wasm_abi(context)
    }
}

impl FromWasmAbi for String {
    type Abi = <Vec<u8> as FromWasmAbi>::Abi;

    fn from_wasm_abi(context: &mut WasmAccess, abi: Self::Abi) -> Result<Self> {
        Ok(Self::from_utf8(Vec::<u8>::from_wasm_abi(context, abi)?)?)
    }
}

impl ToWasmAbi for DynValue {
    type Abi = <&'static [u8] as ToWasmAbi>::Abi;

    fn to_wasm_abi(&self, context: &mut WasmAccess) -> Result<Self::Abi> {
        let mut bytes = Vec::new();
        rmpv::encode::write_value(&mut bytes, self)?;
        println!("Encoded rmpv value: {:?}, {:?}", self, bytes);
        bytes.as_slice().to_wasm_abi(context)
    }
}

impl FromWasmAbi for DynValue {
    type Abi = <Vec<u8> as FromWasmAbi>::Abi;

    fn from_wasm_abi(context: &mut WasmAccess, abi: Self::Abi) -> Result<Self> {
        let bytes = Vec::<u8>::from_wasm_abi(context, abi)?;
        let mut bytes = bytes.as_slice();
        Ok(rmpv::decode::read_value(&mut bytes)?)
    }
}
