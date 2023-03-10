#[cfg(feature = "api-generation")]
use fp_bindgen::{prelude::*, types::CargoDependency};
#[cfg(feature = "api-generation")]
use std::collections::{BTreeMap, BTreeSet};

#[allow(dead_code)]
mod types;
#[cfg(feature = "api-generation")]
use types::*;

#[cfg(feature = "api-generation")]
fp_import! {
    fn level_width() -> i32;
    fn level_height() -> i32;

    fn get_own_position() -> TilePosition;
    fn set_own_position(position: TilePosition);

    fn get_slime_amount(position: TilePosition) -> SlimeAmount;
    fn set_slime_amount(position: TilePosition, amount: SlimeAmount);

    fn is_mouse_pressed(mouse_button: MouseButton) -> bool;
    fn was_mouse_just_pressed(mouse_button: MouseButton) -> bool;
    fn was_mouse_just_released(mouse_button: MouseButton) -> bool;

    fn get_mouse_position() -> Option<Position>;
}

#[cfg(feature = "api-generation")]
fp_export! {
    fn init();

    //fn on_mouse_click(event: MouseEvent);

    fn update(time_elapsed: TimeInterval);
    fn render();
}

#[cfg(feature = "api-generation")]
fn main() {
    // For plugin

    use fp_bindgen::types::CargoDependency;

    let plugin_dependencies = BTreeMap::from([
        (
            "fp-bindgen-support",
            // CargoDependency::with_path_and_features(
            //     "../../../fp-bindgen/fp-bindgen-support",
            //     BTreeSet::from(["async", "guest"]),
            // ),
            CargoDependency {
                path: Some("../../../fp-bindgen/fp-bindgen-support"),
                features: BTreeSet::from(["async", "guest"]),
                ..Default::default()
            },
        ),
        ("derive_more", CargoDependency::with_version("0.99.17")),
    ]);

    fp_bindgen!(BindingConfig {
        // bindings_type: BindingsType::RustPlugin(
        //     RustPluginConfig::builder()
        //         .name("liquislime-api")
        //         .authors(vec!["kajacx".to_owned()])
        //         .version("0.1.0")
        //         .dependencies(plugin_dependencies)
        //         .build()
        // ),
        bindings_type: BindingsType::RustPlugin(RustPluginConfig {
            name: "liquislime-api",
            authors: "[\"kajacx\"]",
            version: "0.1.0",
            dependencies: plugin_dependencies,
        }),
        path: "bindings/rust-plugin",
    });

    // For runtime
    fp_bindgen!(BindingConfig {
        bindings_type: BindingsType::RustWasmerRuntime,
        path: "bindings/rust-wasmer-runtime",
    });
}

#[cfg(not(feature = "api-generation"))]
fn main() {
    panic!("Run the crate with the \"api-generation\" feature enabled.");
}
