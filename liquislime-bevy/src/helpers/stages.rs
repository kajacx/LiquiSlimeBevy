use bevy::{ecs::schedule::ScheduleLabel, prelude::*};

#[derive(Debug, Hash, PartialEq, Eq, Clone, ScheduleLabel, SystemSet)]
pub enum Phase {
    InputRead,
    WasmUpdate,
    GameUpdate,
    WasmRender,
    GameRender,
}

pub struct StagesPlugin;

impl Plugin for StagesPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                Phase::InputRead,
                Phase::WasmUpdate,
                Phase::GameUpdate,
                Phase::WasmRender,
                Phase::GameRender,
            )
                .chain(),
        );
    }
}
