use std::fmt::Debug;

use bevy::prelude::debug;
use js_sys::{Function, Reflect};
use wasm_bindgen::{prelude::*, JsValue};
use wasm_bindgen_futures::JsFuture;

use crate::units::api_spec::{add_slime_amount, get_own_position};

#[derive(Debug)]
pub struct UnitModule {
    module: JsValue,
    instantiate: Function,
    compile_core: JsValue,
    import_object: JsValue,
    _drop_handles: Vec<DropHandler>,
}

#[derive(Debug)]
pub struct UnitInstance {
    instance: JsValue,
    update: Function,
}

// SAFETY: Bevy says it runs on only one "thread" (web worker) on the web
unsafe impl Send for UnitModule {}
unsafe impl Sync for UnitModule {}
unsafe impl Send for UnitInstance {}
unsafe impl Sync for UnitInstance {}

impl UnitModule {
    pub async fn from_bytes(bytes: &[u8]) -> Self {
        let plugin = std::str::from_utf8(bytes).unwrap();
        let path = format!("/webserver/assets/plugins/{plugin}-jco/{plugin}-component.js");

        let module = js_sys::eval(&format!("import('{path}')")).expect("eval import");
        let module = await_js_value(module).await;

        let instantiate = Reflect::get(&module, &"instantiate".into()).expect("get instantiate");
        let instantiate: Function = instantiate.try_into().expect("get as fn");

        let compile_core = js_sys::eval(&format!("(obj, imports) => fetch('/webserver/assets/plugins/{plugin}-jco/' + obj).then(data => data.arrayBuffer()).then(buffer => WebAssembly.compile(buffer, imports))"))
        // let compile_core = js_sys::eval(
        //     r#"async (url, imports) => {
        //     let data = await fetch("/out-dir/" + url);
        //     return WebAssembly.compile(await data.arrayBuffer(), imports);
        //   }"#,
        // )
        .expect("eval compile core");

        // let get_own_position_import = Closure::<dyn Fn() -> TilePosition>::new(|| {
        //     let position = get_own_position();
        //     TilePosition {
        //         x: position.x,
        //         y: position.y,
        //     }
        // });

        // let add_slime_amount_import = Closure::<dyn Fn(TilePosition, SlimeAmount)>::new(
        //     |position: TilePosition, amount: SlimeAmount| {
        //         let position = crate::api::TilePosition::new(position.x, position.y);
        //         let amount = crate::api::SlimeAmount(amount.amount);
        //         add_slime_amount(position, amount);
        //     },
        // );

        let get_own_position_import = Closure::<dyn Fn() -> JsValue>::new(|| {
            let position = get_own_position();
            js_sys::eval(&format!("{{x: {}, y: {}}}", position.x, position.y))
                .expect("eval position")
        });

        let add_slime_amount_import =
            Closure::<dyn Fn(JsValue, JsValue)>::new(|position: JsValue, amount: JsValue| {
                debug!("ADD SLIME AMOUNT: {position:?}, {amount:?}");
            });

        let import_object: JsValue = js_sys::Object::new().into();
        Reflect::set(
            &import_object,
            &"get_own_position".into(),
            &get_own_position_import.as_ref().into(),
        )
        .unwrap();
        Reflect::set(
            &import_object,
            &"add_slime_amount".into(),
            &add_slime_amount_import.as_ref().into(),
        )
        .unwrap();

        let mut drop_handlers = Vec::new();
        drop_handlers.push(DropHandler::new(get_own_position_import));
        drop_handlers.push(DropHandler::new(add_slime_amount_import));

        Self {
            module,
            instantiate,
            compile_core,
            import_object,
            _drop_handles: drop_handlers,
        }
    }

    pub async fn instantiate(&self) -> UnitInstance {
        let instance = self
            .instantiate
            .call2(&self.module, &self.compile_core, &self.import_object)
            .expect("call instantiate");
        let instance = await_js_value(instance).await;

        let update = Reflect::get(&instance, &"update".into()).expect("get update");
        let update: Function = update.try_into().expect("get update as fn");

        UnitInstance { instance, update }
    }
}

impl UnitInstance {
    pub fn update(&self, time_elapsed: crate::api::TimeInterval) {
        // let time_elapsed = TimeInterval {
        //     fragments: time_elapsed.0,
        // };

        let time_elapsed =
            js_sys::eval(&format!("{{fragments: {}}}", time_elapsed.0)).expect("eval time elapsed");

        self.update
            .call1(&self.instance, &time_elapsed.into())
            .unwrap();
    }
}

async fn await_js_value(value: JsValue) -> JsValue {
    let as_promise = js_sys::Promise::try_from(value).expect("value to promise");
    JsFuture::from(as_promise).await.expect("awaiting promise")
}

// #[wasm_bindgen]
// struct TilePosition {
//     x: i32,
//     y: i32,
// }

// #[wasm_bindgen]
// struct SlimeAmount {
//     amount: i64,
// }

// #[wasm_bindgen]
// struct TimeInterval {
//     fragments: i64,
// }

#[derive(Debug)]
struct DropHandler(Box<dyn Debug>);

impl DropHandler {
    pub fn new<T: Debug + 'static>(value: T) -> Self {
        Self(Box::new(value))
    }
}
