use super::{
    helpers::{pack_f32s, pack_u32s, unpack_f32s, unpack_u32s},
    FromWasmAbi, FromWasmAbiSimple, ScriptImpl, StoreData, ToWasmAbi, ToWasmAbiSimple, WasmAccess,
};
use crate::{
    api::{
        ApiFaction, ApiInstance, ApiPosition, ApiSlimeAmount, ApiTilePosition, ApiTimeInterval,
        ApiUnit, DynValue, SettingsDescription, SettingsUiDisplay, SettingsValue,
    },
    components::UnitId,
};
use wasm_bridge::{Result, StoreContextMut};

impl ToWasmAbiSimple for ApiFaction {
    type Abi = u32;

    fn to_wasm_abi_simple(&self) -> Self::Abi {
        self.0 as u32
    }
}

impl FromWasmAbiSimple for ApiFaction {
    type Abi = u32;

    fn from_wasm_abi_simple(abi: Self::Abi) -> Self {
        Self(abi as u8)
    }
}

impl ToWasmAbiSimple for ApiUnit {
    type Abi = u32;

    fn to_wasm_abi_simple(&self) -> Self::Abi {
        self.0 .0
    }
}

impl FromWasmAbiSimple for ApiUnit {
    type Abi = u32;

    fn from_wasm_abi_simple(abi: Self::Abi) -> Self {
        Self(UnitId(abi))
    }
}

impl ToWasmAbiSimple for ApiInstance {
    type Abi = u32;

    fn to_wasm_abi_simple(&self) -> Self::Abi {
        self.0
    }
}

impl FromWasmAbiSimple for ApiInstance {
    type Abi = u32;

    fn from_wasm_abi_simple(abi: Self::Abi) -> Self {
        Self(abi)
    }
}

impl ToWasmAbiSimple for ApiPosition {
    type Abi = u64;

    fn to_wasm_abi_simple(&self) -> Self::Abi {
        pack_f32s(self.x, self.y)
    }
}

impl FromWasmAbiSimple for ApiPosition {
    type Abi = u64;

    fn from_wasm_abi_simple(abi: Self::Abi) -> Self {
        let unpacked = unpack_f32s(abi);

        Self {
            x: unpacked.0,
            y: unpacked.1,
        }
    }
}

impl ToWasmAbiSimple for Option<ApiPosition> {
    type Abi = u64;

    fn to_wasm_abi_simple(&self) -> Self::Abi {
        match &self {
            Some(position) => position.to_wasm_abi_simple(),
            None => pack_f32s(f32::NAN, f32::NAN),
        }
    }
}

impl FromWasmAbiSimple for Option<ApiPosition> {
    type Abi = u64;

    fn from_wasm_abi_simple(abi: Self::Abi) -> Self {
        let unpacked = unpack_f32s(abi);
        if unpacked.0.is_nan() && unpacked.1.is_nan() {
            None
        } else {
            Some(ApiPosition {
                x: unpacked.0,
                y: unpacked.1,
            })
        }
    }
}

impl ToWasmAbiSimple for ApiSlimeAmount {
    type Abi = i64;

    fn to_wasm_abi_simple(&self) -> Self::Abi {
        self.0
    }
}

impl FromWasmAbiSimple for ApiSlimeAmount {
    type Abi = i64;

    fn from_wasm_abi_simple(abi: Self::Abi) -> Self {
        Self(abi)
    }
}

impl ToWasmAbiSimple for ApiTilePosition {
    type Abi = u64;

    fn to_wasm_abi_simple(&self) -> Self::Abi {
        pack_u32s(self.x as u32, self.y as u32)
    }
}

impl FromWasmAbiSimple for ApiTilePosition {
    type Abi = u64;

    fn from_wasm_abi_simple(abi: Self::Abi) -> Self {
        let unpacked = unpack_u32s(abi);

        Self {
            x: unpacked.0 as i32,
            y: unpacked.1 as i32,
        }
    }
}

impl ToWasmAbiSimple for ApiTimeInterval {
    type Abi = i64;

    fn to_wasm_abi_simple(&self) -> Self::Abi {
        self.0
    }
}

impl FromWasmAbiSimple for ApiTimeInterval {
    type Abi = i64;

    fn from_wasm_abi_simple(abi: Self::Abi) -> Self {
        Self(abi)
    }
}

impl ToWasmAbi for DynValue {
    type Abi = <&'static [u8] as ToWasmAbi>::Abi;

    fn to_wasm_abi(&self, context: &mut WasmAccess) -> Result<Self::Abi> {
        let mut bytes = vec![];
        self.serialize(&mut bytes)?;
        bytes.as_slice().to_wasm_abi(context)
    }
}

impl FromWasmAbi for DynValue {
    type Abi = <Vec<u8> as FromWasmAbi>::Abi;

    fn from_wasm_abi(context: &mut WasmAccess, abi: Self::Abi) -> Result<Self> {
        let bytes = Vec::<u8>::from_wasm_abi(context, abi)?;
        DynValue::deserialize(&mut std::io::Cursor::new(bytes))
    }
}

impl ToWasmAbi for SettingsValue {
    type Abi = <DynValue as ToWasmAbi>::Abi;

    fn to_wasm_abi(&self, context: &mut WasmAccess) -> Result<Self::Abi> {
        self.0.to_wasm_abi(context)
    }
}

impl FromWasmAbi for SettingsValue {
    type Abi = <DynValue as FromWasmAbi>::Abi;

    fn from_wasm_abi(context: &mut WasmAccess, abi: Self::Abi) -> Result<Self> {
        Ok(Self(DynValue::from_wasm_abi(context, abi)?))
    }
}

impl FromWasmAbi for SettingsDescription {
    type Abi = <DynValue as FromWasmAbi>::Abi;

    fn from_wasm_abi(context: &mut WasmAccess, abi: Self::Abi) -> Result<Self> {
        Ok(Self::deserialize(&DynValue::from_wasm_abi(context, abi)?))
    }
}
