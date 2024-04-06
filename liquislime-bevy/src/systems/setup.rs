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

pub struct GameSetupPlugin;

impl Plugin for GameSetupPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<resources::InputQueue>();
        app.init_resource::<resources::SelectedUnit>();
        app.init_resource::<resources::MouseState>();

        app.add_systems(Startup, setup_camera);
        app.add_systems(Startup, spawn_tiles);
        app.add_systems(Startup, spawn_sources);
        app.add_systems(Startup, set_window_icon);
        app.add_systems(Startup, setup_selector);

        app.add_plugins(EguiPlugin);
        app.add_systems(Update, setup_egui);
    }
}

fn setup_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    let scale = 0.02f32;
    let position = 5f32;

    camera.transform.scale.x = scale;
    camera.transform.scale.y = scale;

    camera.transform.translation.x = position;
    camera.transform.translation.y = position;

    commands.spawn(camera);
}

fn spawn_tiles(mut commands: Commands) {
    commands.spawn(SlimeGrids::new(WORLD_WIDTH, WORLD_HEIGHT));

    for x in 0..WORLD_WIDTH {
        for y in 0..WORLD_HEIGHT {
            let position = TilePosition {
                x: x as i32,
                y: y as i32,
            };
            let sprite = SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2 { x: 1f32, y: 1f32 }),
                    ..Default::default()
                },
                transform: Transform::from_translation(position.to_position_center().to_vec3(0.0)),
                ..Default::default()
            };

            commands.spawn((TilePositionComponent::from(position), sprite, Tile));
        }
    }
}

fn spawn_sources(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut create_unit = move |faction: Faction,
                                position: TilePosition,
                                texture_file: &'static str,
                                unit_id: UnitId,
                                plugins: &[(&str, Settings)]| {
        let sprite = SpriteBundle {
            texture: asset_server.load(texture_file),
            sprite: Sprite {
                custom_size: Some(Vec2 {
                    x: 0.75f32,
                    y: 0.75f32,
                }),
                ..Default::default()
            },
            transform: Transform::from_translation(position.to_position_center().to_vec3(1.0)),
            ..Default::default()
        };

        let scripts_component = plugins
            .iter()
            .map(|(plugin_filename, settings)| {
                (get_plugin(plugin_filename, &asset_server), settings.clone())
            })
            .collect::<Vec<_>>();

        commands.spawn((
            FactionComponent::from(faction),
            TilePositionComponent::from(position),
            sprite,
            Building,
            ScriptsComponent(scripts_component),
            unit_id,
        ));
    };

    create_unit(
        Faction::new(0),
        crate::api::TilePosition::new(2, 5),
        "tiles_grayscale/tile_0057.png",
        UnitId(1),
        &[
            (
                "liquislime_slime_spawner_plugin.wasm",
                Settings(
                    serde_json::json!({
                        "amount": SlimeAmount::from_integer(100)
                    })
                    .to_string(),
                ),
            ),
            (
                "liquislime_slime_clicker_plugin.wasm",
                Settings(
                    serde_json::json!({
                        "amount": SlimeAmount::from_integer(2000)
                    })
                    .to_string(),
                ),
            ),
        ],
    );

    create_unit(
        Faction::new(1),
        crate::api::TilePosition::new(7, 1),
        "tiles_grayscale/tile_0055.png",
        UnitId(2),
        &[(
            "liquislime_slime_spawner_plugin.wasm",
            Settings(
                serde_json::json!({
                    "amount": SlimeAmount::from_integer(150)
                })
                .to_string(),
            ),
        )],
    );
}

fn get_plugin(plugin_filename: &str, asset_server: &Res<AssetServer>) -> ScriptComponent {
    let path = format!("plugins/{plugin_filename}");
    let handle: Handle<ScriptModule> = asset_server.load(path);

    ScriptComponent::new(handle)
}

// From https://bevy-cheatbook.github.io/window/icon.html
fn set_window_icon(windows: NonSend<WinitWindows>) {
    if cfg!(target_arch = "wasm32") {
        return;
    }

    // here we use the `image` crate to load our icon data from a png file
    // this is not a very bevy-native solution, but it will do
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("assets/icon.png")
            .expect("failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

    // do it for all windows
    for window in windows.windows.values() {
        window.set_window_icon(Some(icon.clone()));
    }
}

fn setup_selector(mut commands: Commands, asset_server: Res<AssetServer>) {
    let sprite = SpriteBundle {
        texture: asset_server.load("icons/selector.png"),
        sprite: Sprite {
            custom_size: Some(Vec2 {
                x: 1.25f32,
                y: 1.25f32,
            }),
            ..Default::default()
        },
        transform: Transform::from_translation(vec3(0.0, 0.0, 2.0)),
        ..Default::default()
    };

    commands.spawn((sprite, SelectorCursor::default()));
}

#[derive(PartialEq)]
enum Enum {
    First,
    Second,
    Third,
}

impl Default for Enum {
    fn default() -> Self {
        Enum::First
    }
}

#[derive(Default)]
struct MyState {
    selection: Enum,
    text: String,
    number: f32,
    flag: bool,
}

fn setup_egui(mut contexts: EguiContexts, mut local: Local<MyState>) {
    // contexts
    // egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
    //     ui.add(egui::Label::new("Hello World!"));
    //     ui.label("A shorter and more convenient way to add a label.");
    //     if ui.button("Click me").clicked() {
    //         // take some action here
    //     }
    // });
    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        ui.label("world");

        ui.label("This is a label");
        ui.hyperlink("https://github.com/emilk/egui");
        ui.text_edit_singleline(&mut local.text);
        if ui.button("Click me").clicked() {}
        ui.add(egui::Slider::new(&mut local.number, 0.0..=100.0));
        ui.add(egui::DragValue::new(&mut local.number));

        ui.checkbox(&mut local.flag, "Checkbox");

        ui.horizontal(|ui| {
            ui.radio_value(&mut local.selection, Enum::First, "First");
            ui.radio_value(&mut local.selection, Enum::Second, "Second");
            ui.radio_value(&mut local.selection, Enum::Third, "Third");
        });

        ui.separator();

        // ui.image((my_image, egui::Vec2::new(640.0, 480.0)));

        ui.collapsing("Click to see what is hidden!", |ui| {
            ui.label("Not much, as it turns out");
        });
    });
}
