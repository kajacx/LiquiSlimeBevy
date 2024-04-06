use bevy::math::vec3;
use bevy::prelude::*;
use bevy::winit::WinitWindows;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use winit::window::Icon;

use crate::assets::ScriptModule;
use crate::components::{
    FactionComponent, ScriptComponent, ScriptsComponent, SelectorCursor, SlimeGrids,
};
use crate::{api::*, WORLD_HEIGHT, WORLD_WIDTH};
use crate::{
    components::{Building, SlimeGrid, Tile, TilePositionComponent},
    resources,
    units::UnitId,
};

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin);
        app.add_systems(Update, display_gui);
    }
}

fn display_gui(mut contexts: EguiContexts) {
    egui::SidePanel::right("right_panel")
        .resizable(true)
        .show(contexts.ctx_mut(), |ui| {
            ui.add(egui::Label::new("Hello World!"));
            ui.label("A shorter and more convenient way to add a label.");
            if ui.button("Click me").clicked() {
                println!("button clicked")
            }
        });
}
