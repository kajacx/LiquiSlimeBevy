use bevy::prelude::*;

use crate::{
    api::ApiFaction,
    components::{Building, SelectorCursor, SlimeGrids, Tile, TilePositionComponent},
    helpers::Phase,
    resources::{GameWindowSpace, SelectedUnit},
    components::UnitId,
};

pub struct GameRenderingPlugin;

impl Plugin for GameRenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, render_slime_color.in_set(Phase::GameRender));
        app.add_systems(Update, update_building_position.in_set(Phase::GameRender));
        app.add_systems(Update, render_selector_cursor.in_set(Phase::GameRender));
        app.add_systems(Update, update_camera_transform_system);
    }
}

fn render_slime_color(
    grid_query: Query<&SlimeGrids>,
    mut tile_query: Query<(&mut Sprite, &TilePositionComponent), With<Tile>>,
) {
    let slime_grid = grid_query
        .get_single()
        .expect("Slime Grid should have been created");

    for (mut sprite, position) in &mut tile_query {
        let amount0 = slime_grid
            .get_amount(ApiFaction::new(0), position.0)
            .as_float() as f32;
        let amount1 = slime_grid
            .get_amount(ApiFaction::new(1), position.0)
            .as_float() as f32;

        let background = Color::rgb(0.6, 0.6, 0.6);
        let color0 = Color::GREEN;
        let color1 = Color::ORANGE;

        if amount0 > 0.0 {
            sprite.color = color_blend(color0, background, amount0 / 100.0 + 0.25);
        } else if amount1 > 0.0 {
            sprite.color = color_blend(color1, background, amount1 / 100.0 + 0.25);
        } else {
            sprite.color = background;
        }
    }
}

fn color_blend(color0: Color, color1: Color, blend: f32) -> Color {
    let blend = blend.clamp(0.0, 1.0);
    Color::rgb(
        color0.r() * blend + color1.r() * (1.0 - blend),
        color0.g() * blend + color1.g() * (1.0 - blend),
        color0.b() * blend + color1.b() * (1.0 - blend),
    )
}

fn update_building_position(
    mut building_query: Query<(&mut Transform, &TilePositionComponent), With<Building>>,
) {
    for (mut transform, position) in &mut building_query {
        let z = transform.translation.z;
        transform.translation = position.0.to_position_center().to_vec3(z);
    }
}

fn render_selector_cursor(
    selected_unit: Res<SelectedUnit>,
    mut selector_cursor: Query<(&mut Transform, &mut Visibility), With<SelectorCursor>>,
    units: Query<(&UnitId, &TilePositionComponent)>,
) {
    let (mut selector_transform, mut visibility) = selector_cursor.single_mut();

    if let Some(id) = selected_unit.0 {
        let position = units
            .iter()
            .find_map(|(unit_id, tile_position)| {
                if *unit_id == id {
                    Some(tile_position.0)
                } else {
                    None
                }
            })
            .expect("find unit by id")
            .to_position_center();

        selector_transform.translation.x = position.x;
        selector_transform.translation.y = position.y;

        // TODO: proper time delta
        selector_transform.rotate_z(-0.01);

        *visibility = Visibility::Visible;
    } else {
        *visibility = Visibility::Hidden;
    }
}

fn update_camera_transform_system(
    occupied_screen_space: Res<GameWindowSpace>,
    windows: Query<&Window>,
    mut camera_query: Query<(&mut OrthographicProjection, &mut Transform)>,
) {
    let (mut projection, mut transform) = camera_query.single_mut();
    let window = windows.single();

    projection.viewport_origin.x = 0.5 - occupied_screen_space.right / window.width() / 2.0;
}
