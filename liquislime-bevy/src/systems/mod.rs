use bevy::{app::PluginGroupBuilder, prelude::*};

use crate::helpers::{RawBytes, RawBytesLoader};

mod input;
mod rendering;
mod setup;
mod update_logic;
mod wasm_update;

pub struct AllGamePlugins;

impl PluginGroup for AllGamePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(CoreGamePlugins)
            .add(setup::GameSetupPlugin)
            .add(input::GameInputPlugin)
            .add(update_logic::UpdateLogicPlugin)
            .add(wasm_update::WasmUpdatePlugin)
            .add(rendering::GameRenderingPlugin)
    }
}

pub struct CoreGamePlugins;

impl Plugin for CoreGamePlugins {
    fn build(&self, app: &mut App) {
        app.add_asset::<RawBytes>()
            .init_asset_loader::<RawBytesLoader>();
    }
}
