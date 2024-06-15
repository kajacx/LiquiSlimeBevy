use try_lock::TryLock;

use super::{LiquislimeImports, LoadedScript, Script};
use crate::{
    api::{
        ApiFaction, ApiInstance, ApiPosition, ApiSlimeAmount, ApiTilePosition, ApiTimeInterval,
        ApiUnit, SettingsValue,
    },
    components::UnitId,
};
use std::{cell::Cell, sync::Arc};

trait LiquislimeImportsStub: Send + Sync + 'static {
    fn level_width(&self) -> i32 {
        10
    }
    fn level_height(&self) -> i32 {
        10
    }

    fn get_current_unit(&self) -> ApiUnit {
        ApiUnit(UnitId(0))
    }
    fn get_current_instance(&self) -> ApiInstance {
        ApiInstance(0)
    }

    fn get_own_faction(&self) -> ApiFaction {
        ApiFaction(0)
    }
    fn get_own_position(&self) -> ApiTilePosition {
        ApiTilePosition::default()
    }

    fn get_slime_amount(&self, _faction: ApiFaction, _position: ApiTilePosition) -> ApiSlimeAmount {
        ApiSlimeAmount::default()
    }
    fn set_slime_amount(
        &self,
        _faction: ApiFaction,
        _position: ApiTilePosition,
        _amount: ApiSlimeAmount,
    ) {
    }

    fn get_mouse_position(&self) -> Option<ApiPosition> {
        None
    }
    fn is_mouse_pressed(&self) -> bool {
        false
    }

    fn log(&self, message: &str) {
        println!("Stub log: {message}");
    }
}

impl<T: LiquislimeImportsStub> LiquislimeImports for T {
    fn level_width(&self) -> i32 {
        <Self as LiquislimeImportsStub>::level_width(self)
    }
    fn level_height(&self) -> i32 {
        <Self as LiquislimeImportsStub>::level_height(self)
    }

    fn get_current_unit(&self) -> ApiUnit {
        <Self as LiquislimeImportsStub>::get_current_unit(self)
    }
    fn get_current_instance(&self) -> ApiInstance {
        <Self as LiquislimeImportsStub>::get_current_instance(self)
    }

    fn get_own_faction(&self) -> ApiFaction {
        <Self as LiquislimeImportsStub>::get_own_faction(self)
    }
    fn get_own_position(&self) -> ApiTilePosition {
        <Self as LiquislimeImportsStub>::get_own_position(self)
    }

    fn get_slime_amount(&self, faction: ApiFaction, position: ApiTilePosition) -> ApiSlimeAmount {
        <Self as LiquislimeImportsStub>::get_slime_amount(self, faction, position)
    }
    fn set_slime_amount(
        &self,
        faction: ApiFaction,
        position: ApiTilePosition,
        amount: ApiSlimeAmount,
    ) {
        <Self as LiquislimeImportsStub>::set_slime_amount(self, faction, position, amount)
    }

    fn get_mouse_position(&self) -> Option<ApiPosition> {
        <Self as LiquislimeImportsStub>::get_mouse_position(self)
    }
    fn is_mouse_pressed(&self) -> bool {
        <Self as LiquislimeImportsStub>::is_mouse_pressed(self)
    }

    fn log(&self, message: &str) {
        <Self as LiquislimeImportsStub>::log(self, message)
    }
}

static SPAWNER_BYTES: &[u8] = include_bytes!(
    "../../../../liquislime-scripts/rust/target/wasm32-unknown-unknown/debug/slime_spawner.wasm"
);

#[derive(Debug, Default, Clone)]
struct LastAmountImports(Arc<TryLock<ApiSlimeAmount>>);

impl LiquislimeImportsStub for LastAmountImports {
    fn set_slime_amount(
        &self,
        _faction: ApiFaction,
        _position: ApiTilePosition,
        amount: ApiSlimeAmount,
    ) {
        *self.0.try_lock().unwrap() = amount;
    }
}

#[test]
fn test_settings() {
    let imports = LastAmountImports::default();

    // TODO: Bad design
    let script = Script::from_bytes("Spawner".into(), SPAWNER_BYTES, imports.clone()).unwrap();
    let spawner_script = LoadedScript::from_bytes(SPAWNER_BYTES, imports.clone()).unwrap();

    let mut instance = spawner_script
        .new_instance(
            script,
            SettingsValue(ApiSlimeAmount::from_integer(100).into()),
        )
        .unwrap();

    instance.update(ApiTimeInterval::from_seconds(1.0)).unwrap();

    assert_eq!(
        *imports.0.try_lock().unwrap(),
        ApiSlimeAmount::from_integer(100)
    );

    instance.with_settings(|settings| {
        *settings.current_settings = SettingsValue(ApiSlimeAmount::from_integer(200).into());
    });
    instance.change_settings().unwrap();

    instance.update(ApiTimeInterval::from_seconds(1.0)).unwrap();

    assert_eq!(
        *imports.0.try_lock().unwrap(),
        ApiSlimeAmount::from_integer(200)
    );
}
