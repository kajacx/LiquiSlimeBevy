use bevy::{app::PluginGroupBuilder, prelude::PluginGroup};

mod rendering;
mod setup;
mod update_logic;

pub struct AllGamePlugins;

impl PluginGroup for AllGamePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(setup::GameSetupPlugin)
            .add(update_logic::UpdateLogicPlugin)
            .add(rendering::GameRenderingPlugin)
    }
}
