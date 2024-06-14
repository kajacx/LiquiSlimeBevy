use crate::components::{FactionComponent, ScriptInstances, SelectorCursor, SlimeGrids};
use crate::resources::{GameWindowSpace, SelectedUnit};
use crate::{api::*, WORLD_HEIGHT, WORLD_WIDTH};
use crate::{
    components::UnitId,
    components::{Building, SlimeGrid, Tile, TilePositionComponent},
    resources,
};
use bevy::math::vec3;
use bevy::prelude::*;
use bevy::winit::WinitWindows;
use bevy_egui::egui::Ui;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use winit::window::Icon;

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
    mut units: Query<(&UnitId, &mut ScriptInstances)>,
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

                let mut scripts = units
                    .iter_mut()
                    .find_map(|(unit_id, scripts)| {
                        if *unit_id == selected_unit_id {
                            Some(scripts)
                        } else {
                            None
                        }
                    })
                    .expect("find unit by id");

                for script in scripts.0.iter_mut() {
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

fn display_script_settings(ui: &mut Ui, instance: &mut ScriptInstance) {
    instance.with_name(|name| {
        ui.label(name);
    });

    let mut changed_settings = false;
    instance.with_settings(|settings| {
        settings
            .settings_description
            .display_ui_element(ui, settings.temp_settings);

        if ui.button("Reset").clicked() {
            *settings.current_settings = settings.default_settings.clone();
            settings
                .settings_description
                .reset_settings(settings.current_settings, settings.temp_settings);
            changed_settings = true;
        }
        if ui.button("Clear").clicked() {
            settings
                .settings_description
                .reset_settings(settings.current_settings, settings.temp_settings);
        }
        if ui.button("Save").clicked() {
            settings
                .settings_description
                .save_settings(settings.temp_settings, settings.current_settings);
            changed_settings = true;
        }
    });

    if (changed_settings) {
        instance.change_settings().expect("TODO: user error");
    }

    ui.separator();
}
