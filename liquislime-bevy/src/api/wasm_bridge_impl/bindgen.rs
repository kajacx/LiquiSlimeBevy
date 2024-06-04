use std::fmt::Debug;

use wasm_bridge::Result;

wasm_bridge::component::bindgen!({
    path: "../protocol.wit",
    world: "liquislime-unit",
});

pub struct LiquislimeHost;

impl LiquislimeUnitImports for LiquislimeHost {
    fn level_width(&mut self) -> Result<i32> {
        Ok(level_width())
    }

    fn level_height(&mut self) -> Result<i32> {
        Ok(level_height())
    }

    fn get_own_position(&mut self) -> Result<TilePosition> {
        Ok(get_own_position().into())
    }

    fn get_own_faction(&mut self) -> Result<Faction> {
        Ok(get_own_faction().into())
    }

    fn get_slime_amount(
        &mut self,
        faction: Faction,
        position: TilePosition,
    ) -> Result<SlimeAmount> {
        Ok(get_slime_amount(faction.into(), position.into()).into())
    }

    fn set_slime_amount(
        &mut self,
        faction: Faction,
        position: TilePosition,
        amount: SlimeAmount,
    ) -> Result<()> {
        set_slime_amount(faction.into(), position.into(), amount.into());
        Ok(())
    }

    fn get_mouse_position(&mut self) -> Result<Option<Position>> {
        Ok(get_mouse_position().map(Into::into))
    }

    fn is_mouse_pressed(&mut self, _button: MouseInput) -> Result<bool> {
        Ok(is_mouse_pressed())
    }
}

impl liquislime::protocol::types::Host for LiquislimeHost {}

impl Debug for LiquislimeHost {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LiquislimeHost cannot derive Debug")
    }
}
