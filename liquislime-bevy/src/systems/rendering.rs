use bevy::prelude::*;

use crate::{
    components::{Building, SlimeGrid, Tile, TilePositionComponent},
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
    grid_query: Query<&SlimeGrid>,
    mut tile_query: Query<(&mut Sprite, &TilePositionComponent), With<Tile>>,
) {
    let slime_grid = grid_query
        .get_single()
        .expect("Slime Grid should have been created");

    for (mut sprite, position) in &mut tile_query {
        let amount = slime_grid.get_amount(position.0.x as usize, position.0.y as usize);
        let rgb = amount.as_integer() as u8;
        sprite.color = Color::rgb_u8(rgb / 4, rgb, rgb / 2);
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
