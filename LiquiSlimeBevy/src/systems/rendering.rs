use bevy::prelude::*;

use crate::components::{Building, SlimeGrid, Tile, TilePosition};

pub struct GameRenderingPlugin;

impl Plugin for GameRenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::Last, render_slime_color);
        app.add_system_to_stage(CoreStage::Last, update_building_postion);
    }
}

fn render_slime_color(
    grid_query: Query<&SlimeGrid>,
    mut tile_query: Query<(&mut Sprite, &TilePosition), With<Tile>>,
) {
    let slime_grid = grid_query
        .get_single()
        .expect("Slime Grid should have been created");

    for (mut sprite, position) in &mut tile_query {
        let amount = slime_grid.get_amount(position.x as usize, position.y as usize);
        let rgb = amount.as_integer() as u8;
        sprite.color = Color::rgb_u8(rgb, rgb, rgb);
    }
}

fn update_building_postion(
    mut building_query: Query<(&mut Transform, &TilePosition), With<Building>>,
) {
    for (mut transform, position) in &mut building_query {
        let z = transform.translation.z;
        transform.translation = position.to_centered_vec(z);
    }
}
