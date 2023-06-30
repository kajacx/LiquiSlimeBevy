use bevy::{app::PluginGroupBuilder, prelude::*};

use crate::helpers::{AssetsGamePlugins, StagesPlugin};

// mod input;
mod rendering;
mod setup;
mod update_logic;
mod wasm_update;

pub struct AllGamePlugins;

impl PluginGroup for AllGamePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(AssetsGamePlugins)
            .add(StagesPlugin)
            .add(setup::GameSetupPlugin)
            // .add(input::GameInputPlugin)
            .add(update_logic::UpdateLogicPlugin)
            .add(wasm_update::WasmUpdatePlugin)
            .add(rendering::GameRenderingPlugin)
    }
}
