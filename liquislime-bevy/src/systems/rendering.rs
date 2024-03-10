use bevy::prelude::*;

use crate::{
    components::{Building, SlimeGrids, Tile, TilePositionComponent},
    helpers::Phase,
};

pub struct GameRenderingPlugin;

impl Plugin for GameRenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, render_slime_color.in_set(Phase::GameRender));
        app.add_systems(Update, update_building_position.in_set(Phase::GameRender));
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
        let amount0 = slime_grid.get_amount(0, position.0).as_integer();
        let amount1 = slime_grid.get_amount(1, position.0).as_integer();

        if amount0 > 0 {
            let rgb = amount0.clamp(0, 255) as u8;
            sprite.color = Color::rgb_u8(rgb / 4, rgb, rgb / 2);
        } else if amount1 > 0 {
            let rgb = amount1.clamp(0, 255) as u8;
            sprite.color = Color::rgb_u8(rgb, rgb / 2, rgb / 4);
        } else {
            sprite.color = Color::BLACK;
        }
    }
}

fn update_building_position(
    mut building_query: Query<(&mut Transform, &TilePositionComponent), With<Building>>,
) {
    for (mut transform, position) in &mut building_query {
        let z = transform.translation.z;
        transform.translation = position.0.to_position_center().to_vec3(z);
    }
}
