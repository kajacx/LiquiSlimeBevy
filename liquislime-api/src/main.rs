use fp_bindgen::{prelude::*, types::CargoDependency};
use std::collections::{BTreeMap, BTreeSet};

mod types;
use types::*;

fp_import! {
    fn level_width() -> i32;
    fn level_height() -> i32;

    fn get_own_position() -> TilePosition;
    fn set_own_position(position: TilePosition);

    fn get_slime_amount(position: TilePosition) -> SlimeAmount;
    fn set_slime_amount(position: TilePosition, SlimeAmount);

    fn was_mouse_just_pressed(button: MouseButton) -> bool;
    fn is_mouse_pressed(button: MouseButton) -> bool;
    fn was_mouse_just_released(button: MouseButton) -> bool;

    fn get_mouse_position() -> Position;
}

fp_export! {
    fn init();

    //fn on_mouse_click(event: MouseEvent);

    fn update(elapsed: TimeInterval);
    fn render();
}

fn main() {
    // For plugin
    fp_bindgen!(BindingConfig {
        bindings_type: BindingsType::RustPlugin(RustPluginConfig {
            name: "fp-protocol",
            authors: "[\"kajacx\"]",
            version: "0.1.0",
            dependencies: BTreeMap::from([
                (
                    "fp-bindgen-support",
                    CargoDependency::with_version_and_features(
                        "2.4.0",
                        BTreeSet::from(["async", "guest"])
                    )
                ),
                ("derive_more", CargoDependency::with_version("0.99.17"))
            ]),
        }),
        path: "bindings/rust-plugin",
    });

    // For runtime
    fp_bindgen!(BindingConfig {
        bindings_type: BindingsType::RustWasmerRuntime,
        path: "bindings/rust-wasmer-runtime",
    });
}
