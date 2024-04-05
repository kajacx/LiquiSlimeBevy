use bevy::{ecs::schedule::ScheduleLabel, prelude::*};

#[derive(Debug, Hash, PartialEq, Eq, Clone, ScheduleLabel, SystemSet)]
pub enum Phase {
    AssetLoad,
    InputRead,
    InputProcess,
    WasmUpdate,
    GameUpdate,
    WasmRender,
    GameRender,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, ScheduleLabel, SystemSet)]
pub struct CompileInput;

pub struct StagesPlugin;

impl Plugin for StagesPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                Phase::InputRead,
                Phase::InputProcess,
                Phase::WasmUpdate,
                Phase::GameUpdate,
                Phase::WasmRender,
                Phase::GameRender,
            )
                .chain(),
        );
        app.configure_sets(Update, (Phase::AssetLoad, Phase::WasmUpdate).chain());
        app.configure_sets(Update, CompileInput);
    }
}
