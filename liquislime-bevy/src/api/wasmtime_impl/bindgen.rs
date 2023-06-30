use std::fmt::Debug;

use crate::units::api_spec::{add_slime_amount, get_own_position};

wasmtime::component::bindgen!({
    path: "../protocol.wit",
    world: "liquislime-unit"
});

#[derive(Debug)]
pub struct LiquislimeHost;

impl LiquislimeUnitImports for LiquislimeHost {
    fn get_own_position(&mut self) -> wasmtime::Result<TilePosition> {
        let position = get_own_position();
        Ok(TilePosition {
            x: position.x,
            y: position.y,
        })
    }

    fn add_slime_amount(
        &mut self,
        position: TilePosition,
        amount: SlimeAmount,
    ) -> wasmtime::Result<()> {
        let position = crate::api::TilePosition {
            x: position.x,
            y: position.y,
        };
        let amount = crate::api::SlimeAmount(amount.amount);
        add_slime_amount(position, amount);
        Ok(())
    }
}
