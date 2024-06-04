use super::{
    engine::get_engine, FromWasmAbi, FromWasmAbiSimple, StoreData, ToWasmAbi, ToWasmAbiSimple,
};
use crate::{
    api::{
        get_mouse_position, get_own_faction, get_own_position, get_slime_amount, is_mouse_pressed,
        level_height, level_width,
        no_bindgen_impl::{store, WasmAccess},
        set_slime_amount, ApiFaction, ApiSlimeAmount, ApiTilePosition,
    },
    helpers::SS,
    units::global_storage::{get_current_instance, get_current_unit},
};
use once_cell::sync::Lazy;
use wasm_bridge::{AsContextMut, Caller, Linker, Result, TypedFunc};

static LINKER: Lazy<SS<Linker<StoreData>>> = Lazy::new(|| SS(create_linker().unwrap()));

pub fn get_linker() -> &'static Linker<StoreData> {
    &LINKER.0
}

fn create_linker() -> Result<Linker<StoreData>> {
    let mut linker = Linker::<StoreData>::new(get_engine());

    linker.func_wrap("liquislime_api", "level_width", |_: Caller<StoreData>| {
        level_width()
    })?;
    linker.func_wrap("liquislime_api", "level_height", |_: Caller<StoreData>| {
        level_height()
    })?;

    linker.func_wrap(
        "liquislime_api",
        "get_current_unit",
        |_: Caller<StoreData>| get_current_unit().to_wasm_abi_simple(),
    )?;
    linker.func_wrap(
        "liquislime_api",
        "get_current_instance",
        |_: Caller<StoreData>| get_current_instance().to_wasm_abi_simple(),
    )?;

    linker.func_wrap(
        "liquislime_api",
        "get_own_faction",
        |_: Caller<StoreData>| get_own_faction().to_wasm_abi_simple(),
    )?;
    linker.func_wrap(
        "liquislime_api",
        "get_own_position",
        |_: Caller<StoreData>| get_own_position().to_wasm_abi_simple(),
    )?;

    linker.func_wrap(
        "liquislime_api",
        "get_slime_amount",
        |_: Caller<StoreData>,
         faction: <ApiFaction as FromWasmAbi>::Abi,
         position: <ApiTilePosition as FromWasmAbi>::Abi| {
            get_slime_amount(
                ApiFaction::from_wasm_abi_simple(faction),
                ApiTilePosition::from_wasm_abi_simple(position),
            )
            .to_wasm_abi_simple()
        },
    )?;
    linker.func_wrap(
        "liquislime_api",
        "set_slime_amount",
        |_: Caller<StoreData>,
         faction: <ApiFaction as FromWasmAbi>::Abi,
         position: <ApiTilePosition as FromWasmAbi>::Abi,
         amount: <ApiSlimeAmount as FromWasmAbi>::Abi| {
            set_slime_amount(
                ApiFaction::from_wasm_abi_simple(faction),
                ApiTilePosition::from_wasm_abi_simple(position),
                ApiSlimeAmount::from_wasm_abi_simple(amount),
            );
        },
    )?;

    linker.func_wrap(
        "liquislime_api",
        "get_mouse_position",
        |_: Caller<StoreData>| get_mouse_position().to_wasm_abi_simple(),
    )?;
    linker.func_wrap(
        "liquislime_api",
        "is_mouse_pressed",
        |_: Caller<StoreData>| is_mouse_pressed().to_wasm_abi_simple(),
    )?;

    linker.func_wrap(
        "liquislime_api",
        "log",
        |mut caller: Caller<StoreData>, msg: <String as FromWasmAbi>::Abi| {
            let msg = String::from_wasm_abi(
                &mut WasmAccess {
                    store: caller.as_context_mut(),
                },
                msg,
            )
            .expect("TODO: user error");
            // TODO: log on web as well
            println!("MSG from script: {msg}");
        },
    )?;

    Ok(linker)
}
