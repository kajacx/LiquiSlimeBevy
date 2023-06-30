use bevy::prelude::*;

use crate::{
    components::{Building, SlimeGrid, Tile, TilePositionComponent},
    helpers::{RenderSync, ToVec3},
};

pub struct GameRenderingPlugin;

impl Plugin for GameRenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(render_slime_color.in_base_set(RenderSync));
        app.add_system(update_building_postion.in_base_set(RenderSync));
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
        // let rgb = amount.as_integer() as u8;
        let rgb = amount.amount as u8;
        sprite.color = Color::rgb_u8(rgb, rgb, rgb);
    }
}

fn update_building_postion(
    mut building_query: Query<(&mut Transform, &TilePositionComponent), With<Building>>,
) {
    for (mut transform, position) in &mut building_query {
        let z = transform.translation.z;
        transform.translation = position.0.to_position_center().to_vec3(z);
    }
}
