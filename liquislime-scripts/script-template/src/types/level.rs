use super::*;
use crate::api::FromWasmAbi;

pub struct Level;

impl Level {
    pub fn width() -> i32 {
        unsafe { i32::from_wasm_abi(crate::api::level_width()) }
    }

    pub fn height() -> i32 {
        unsafe { i32::from_wasm_abi(crate::api::level_height()) }
    }
}
