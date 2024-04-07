use bevy::math::vec3;
use bevy::prelude::*;
use bevy::winit::WinitWindows;
use bevy_egui::egui::Ui;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use winit::window::Icon;

use crate::assets::ScriptModule;
use crate::components::{
    FactionComponent, ScriptHolder, ScriptsComponent, SelectorCursor, SlimeGrids,
};
use crate::resources::{GameWindowSpace, SelectedUnit};
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

fn display_gui(
    mut contexts: EguiContexts,
    mut game_window_space: ResMut<GameWindowSpace>,
    selected_unit: Res<SelectedUnit>,
    units: Query<(&UnitId, &ScriptsComponent)>,
) {
    game_window_space.right = egui::SidePanel::right("right_panel")
        .resizable(true)
        .show(contexts.ctx_mut(), |ui| {
            ui.set_min_size(ui.available_size_before_wrap());

            let selected_unit_id = selected_unit.0;
            if let Some(selected_unit_id) = selected_unit_id {
                // TODO: heap allocation on every frame?
                ui.label(format!("Selected unit: {}", selected_unit_id));
                ui.separator();

                let scripts = units
                    .iter()
                    .find_map(|(unit_id, scripts)| {
                        if *unit_id == selected_unit_id {
                            Some(scripts)
                        } else {
                            None
                        }
                    })
                    .expect("find unit by id");

                for script in scripts.0.iter() {
                    display_script_settings(ui, script);
                }
            } else {
                ui.label("No unit selected.");
            }
        })
        .response
        .rect
        .width();
}

fn display_script_settings(ui: &mut Ui, script: &ScriptHolder) {
    ui.label(script.name);
    script.with_settings(|settings| {
        if let Some((description, value, tmp_value)) = settings {
            description.display_ui_element(ui, tmp_value);
            if ui.button("Reset").clicked() {
                description.reset_settings(value, tmp_value);
            }
            if ui.button("Save").clicked() {
                description.save_settings(tmp_value, value);
            }
        } else {
            ui.label("Script is loading...");
        }
    });
    ui.separator();
}

pub trait SettingsUiDisplay {
    fn display_ui_element(&self, ui: &mut Ui, value: &mut SettingsTempValue);

    fn save_settings(&self, tmp_value: &SettingsTempValue, value: &mut SettingsValue);

    fn reset_settings(&self, value: &SettingsValue, tmp_value: &mut SettingsTempValue);
}
