use bevy::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
#[system_set(base)]
pub struct InputRead;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
#[system_set(base)]
pub enum WasmUpdate {
    Update,
    Render,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
#[system_set(base)]
pub struct RenderSync;

pub struct StagesPlugin;

impl Plugin for StagesPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            (
                CoreSet::PreUpdateFlush,
                InputRead,
                WasmUpdate::Update,
                CoreSet::Update,
            )
                .chain(),
        );
        app.configure_sets(
            (
                CoreSet::UpdateFlush,
                WasmUpdate::Render,
                RenderSync,
                CoreSet::PostUpdate,
            )
                .chain(),
        );
    }
}
