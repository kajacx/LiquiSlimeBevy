use std::fmt::Debug;

use crate::units::{api_spec::*, global_storage::get_mouse_state};

use wasm_bridge::Result;

wasm_bridge::component::bindgen!({
    path: "../protocol.wit",
    world: "liquislime-unit",
});

pub struct LiquislimeHost;

impl LiquislimeUnitImports for LiquislimeHost {
    fn get_own_position(&mut self) -> Result<TilePosition> {
        Ok(get_own_position().into())
    }

    fn add_slime_amount(&mut self, position: TilePosition, amount: SlimeAmount) -> Result<()> {
        add_slime_amount(position.into(), amount.into());
        Ok(())
    }

    fn get_mouse_position(&mut self) -> Result<Option<Position>> {
        Ok(get_mouse_state().position.map(Into::into))
    }

    fn is_mouse_pressed(&mut self, _button: MouseInput) -> Result<bool> {
        Ok(is_mouse_pressed())
    }
}

impl Debug for LiquislimeHost {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LiquislimeHost cannot derive Debug")
    }
}
