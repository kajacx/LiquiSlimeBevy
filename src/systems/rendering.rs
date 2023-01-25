use bevy::prelude::*;

pub struct GameRenderingPlugin;

impl Plugin for GameRenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::Last, render_slime_color);
    }
}

fn render_slime_color(
    grid_query: Query<&SlimeGrid>,
    mut tile_query: Query<(&mut Sprite, &TilePosition)>,
) {
    let slime_grid = grid_query.single();
    for (mut sprite, position) in &mut tile_query {
        let amount = slime_grid.get_amount(position.x as usize, position.y as usize);
        let rgb = amount.as_integer() as u8;
        sprite.color = Color::rgb_u8(rgb, rgb, rgb);
    }
}
