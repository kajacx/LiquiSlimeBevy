trait WasmRawType {
    fn get_type_name() -> &'static str;
}

impl WasmRawType for u64 {
    fn get_type_name() -> &'static str {
        "u64"
    }
}

impl WasmRawType for u32 {
    fn get_type_name() -> &'static str {
        "u32"
    }
}

mod carriers;
mod impls;
mod traits;
