use ::glam::IVec2;
use macroquad::{input, prelude::*};

use crate::liquislime::{Faction, SlimeAmount};

mod liquislime;

#[macroquad::main("BasicShapes")]
async fn main() {
    let faction0 = Faction::new(0);
    let faction1 = Faction::new(1);

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

        clear_background(RED);

        draw_slime_grid(&state.grids.grids[0], 0.0, 0.0, 10.0);
        draw_slime_grid(&state.grids.grids[1], 600.0, 0.0, 10.0);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}

fn draw_slime_grid(grid: &liquislime::SlimeGrid, offset_x: f32, offset_y: f32, tile_size: f32) {
    for y in 0..grid.height {
        for x in 0..grid.width {
            let amount = grid.get_amount(liquislime::TilePosition::new(x as _, y as _));
            let amount = amount.as_float() / 1000.0;
            let color_value = (amount.clamp(0.0, 1.0) * 255.0) as u8;
            let color = Color::from_rgba(color_value, color_value, color_value, 255);
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
