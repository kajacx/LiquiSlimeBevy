use bevy::{app::PluginGroupBuilder, prelude::*};

use crate::assets::AssetsGamePlugin;
use crate::helpers::StagesPlugin;

// mod compile_input;
// pub mod gui;
mod input_process;
mod input_read;
mod rendering;
mod setup;
mod update_logic;
mod wasm_update;

pub struct AllGamePlugins;

impl PluginGroup for AllGamePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(AssetsGamePlugin)
            .add(StagesPlugin)
            .add(setup::GameSetupPlugin)
            // .add(gui::GuiPlugin)
            .add(input_read::InputReadPlugin)
            .add(input_process::InputProcessPlugin)
            .add(update_logic::UpdateLogicPlugin)
            .add(wasm_update::WasmUpdatePlugin)
            .add(rendering::GameRenderingPlugin)
        // .add(compile_input::CompileInputPlugin)
    }
}
