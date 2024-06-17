use super::{
    engine::get_engine, FromWasmAbi, FromWasmAbiSimple, StoreData, ToWasmAbi, ToWasmAbiSimple,
};
use crate::{
    api::{
        no_bindgen_impl::WasmAccess, ApiFaction, ApiSlimeAmount, ApiTilePosition, LiquislimeImports,
    },
    units::global_storage::{get_current_instance, get_current_unit},
};
use std::sync::Arc;
use wasm_bridge::{AsContextMut, Caller, Linker, Result, TypedFunc};

pub fn create_linker(imports: impl LiquislimeImports) -> Result<Linker<StoreData>> {
    let imports = Arc::new(imports);
    let mut linker = Linker::<StoreData>::new(get_engine());

    let imports_clone = imports.clone();
    linker.func_wrap(
        "liquislime_api",
        "level_width",
        move |_: Caller<StoreData>| imports_clone.level_width(),
    )?;

    let imports_clone = imports.clone();
    linker.func_wrap(
        "liquislime_api",
        "level_height",
        move |_: Caller<StoreData>| imports_clone.level_height(),
    )?;

    let imports_clone = imports.clone();
    linker.func_wrap(
        "liquislime_api",
        "get_current_unit",
        move |_: Caller<StoreData>| imports_clone.get_current_unit().to_wasm_abi_simple(),
    )?;

    let imports_clone = imports.clone();
    linker.func_wrap(
        "liquislime_api",
        "get_current_instance",
        move |_: Caller<StoreData>| imports_clone.get_current_instance().to_wasm_abi_simple(),
    )?;

    let imports_clone = imports.clone();
    linker.func_wrap(
        "liquislime_api",
        "get_own_faction",
        move |_: Caller<StoreData>| imports_clone.get_own_faction().to_wasm_abi_simple(),
    )?;

    let imports_clone = imports.clone();
    linker.func_wrap(
        "liquislime_api",
        "get_own_position",
        move |_: Caller<StoreData>| imports_clone.get_own_position().to_wasm_abi_simple(),
    )?;

    let imports_clone = imports.clone();
    linker.func_wrap(
        "liquislime_api",
        "get_slime_amount",
        move |_: Caller<StoreData>,
              faction: <ApiFaction as FromWasmAbi>::Abi,
              position: <ApiTilePosition as FromWasmAbi>::Abi| {
            imports_clone
                .get_slime_amount(
                    ApiFaction::from_wasm_abi_simple(faction),
                    ApiTilePosition::from_wasm_abi_simple(position),
                )
                .to_wasm_abi_simple()
        },
    )?;

    let imports_clone = imports.clone();
    linker.func_wrap(
        "liquislime_api",
        "set_slime_amount",
        move |_: Caller<StoreData>,
              faction: <ApiFaction as FromWasmAbi>::Abi,
              position: <ApiTilePosition as FromWasmAbi>::Abi,
              amount: <ApiSlimeAmount as FromWasmAbi>::Abi| {
            imports_clone.set_slime_amount(
                ApiFaction::from_wasm_abi_simple(faction),
                ApiTilePosition::from_wasm_abi_simple(position),
                ApiSlimeAmount::from_wasm_abi_simple(amount),
            );
        },
    )?;

    let imports_clone = imports.clone();
    linker.func_wrap(
        "liquislime_api",
        "get_mouse_position",
        move |_: Caller<StoreData>| imports_clone.get_mouse_position().to_wasm_abi_simple(),
    )?;

    let imports_clone = imports.clone();
    linker.func_wrap(
        "liquislime_api",
        "is_mouse_pressed",
        move |_: Caller<StoreData>| imports_clone.is_mouse_pressed().to_wasm_abi_simple(),
    )?;

    let imports_clone = imports.clone();
    linker.func_wrap(
        "liquislime_api",
        "log",
        move |mut caller: Caller<StoreData>, message: <String as FromWasmAbi>::Abi| {
            let mut context = WasmAccess {
                store: caller.as_context_mut(),
            };
            imports_clone
                .log(&String::from_wasm_abi(&mut context, message).expect("TODO: User error"));
        },
    )?;

    linker.func_wrap(
        "env",
        "abort",
        |caller: Caller<StoreData>, msg_ptr: i32, file_ptr: i32, line: i32, column: i32| {
            let msg = caller
                .data()
                .current_script
                .read_zero_terminated_string(&caller, msg_ptr as usize)
                .expect("TODO: user error");
            let file = caller
                .data()
                .current_script
                .read_zero_terminated_string(&caller, file_ptr as usize)
                .expect("TODO: user error");
            // TODO: print on web
            println!("Script abort: {msg}");
            println!("In {file}:{line}:{column}");
        },
    )?;

    Ok(linker)
}
