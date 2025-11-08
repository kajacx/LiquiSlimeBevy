use ::glam::IVec2;
use macroquad::{input, prelude::*};

use crate::liquislime::{Faction, SlimeAmount};

mod liquislime;

#[macroquad::main("BasicShapes")]
async fn main() {
    let faction0 = Faction::new(0, GREEN);
    let faction1 = Faction::new(1, ORANGE);

    let mut state = liquislime::GameState::new(50, 50);

    state
        .grids
        .set_amount(faction0, IVec2::new(0, 1), SlimeAmount::from_integer(50000));

    state
        .grids
        .set_amount(faction1, IVec2::new(8, 5), SlimeAmount::from_integer(60000));

    loop {
        state.update(liquislime::TimeInterval::from_seconds(
            get_frame_time() as f64
        ));

        if input::is_mouse_button_pressed(MouseButton::Left) {
            #[allow(unused_must_use)]
            state.grids.try_add_amount(
                faction0,
                liquislime::TilePosition::new(
                    (input::mouse_position().0 as i32) / 10,
                    (input::mouse_position().1 as i32) / 10,
                ),
                SlimeAmount::from_integer(1000000),
            );
        }

        if input::is_mouse_button_down(MouseButton::Right) {
            #[allow(unused_must_use)]
            state.grids.try_add_amount(
                faction1,
                liquislime::TilePosition::new(
                    (input::mouse_position().0 as i32) / 10,
                    (input::mouse_position().1 as i32) / 10,
                ),
                SlimeAmount::from_integer(10000),
            );
        }

        clear_background(LIGHTGRAY);

        draw_slime_grid(&state.grids.grids[0], 0.0, 0.0, 10.0, faction0.color());
        draw_slime_grid(&state.grids.grids[1], 0.0, 0.0, 10.0, faction1.color());
        // draw_slime_grid_factions(&state.grids, 0.0, 0.0, 20.0, &[faction0, faction1]);

        // draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        // draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        // draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        // draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}

fn draw_slime_grid(
    grid: &liquislime::SlimeGrid,
    offset_x: f32,
    offset_y: f32,
    tile_size: f32,
    color: Color,
) {
    for y in 0..grid.height {
        for x in 0..grid.width {
            let amount = grid.get_amount(liquislime::TilePosition::new(x as _, y as _));
            let amount = amount.as_float() / 1000.0;
            let alpha_value = amount.clamp(0.0, 1.0);
            let slime_color = color.with_alpha(alpha_value);
            draw_rectangle(
                offset_x + x as f32 * tile_size,
                offset_y + y as f32 * tile_size,
                tile_size,
                tile_size,
                slime_color,
            );
        }
    }
}

// Write a function that will draw the different slime factions in different colors
fn draw_slime_grid_factions(
    grids: &liquislime::SlimeGrids,
    offset_x: f32,
    offset_y: f32,
    tile_size: f32,
    factions: &[Faction],
) {
    for y in 0..grids.height() {
        for x in 0..grids.width() {
            let position = liquislime::TilePosition::new(x as _, y as _);
            let mut total_amount = 0.0;
            let mut color = Color::from_rgba(0, 0, 0, 255); // Default to black if no slime is present
            for faction in factions {
                let amount = grids.get_amount(*faction, position);
                total_amount += amount.as_float() as f32;
                let amount_ratio = (amount.as_float() as f32 / 1000.0).clamp(0.0, 1.0);
                color.r += faction.color().r * amount_ratio;
                color.g += faction.color().g * amount_ratio;
                color.b += faction.color().b * amount_ratio;
            }
            if total_amount > 0.0 {
                color.r = (color.r / total_amount).min(1.0);
                color.g = (color.g / total_amount).min(1.0);
                color.b = (color.b / total_amount).min(1.0);
            }
            draw_rectangle(
                offset_x + x as f32 * tile_size,
                offset_y + y as f32 * tile_size,
                tile_size,
                tile_size,
                color,
            );
        }
    }
}
