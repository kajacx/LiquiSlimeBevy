use crate::input::{InputHelper, InputQueryImpl};
use liquislime_core::{
    Faction, FullState, GameState, Screen, SlimeAmount, TilePosition, TimeInterval,
};
use macroquad::prelude::*;

mod examples;
mod input;
mod render;
mod setup;
mod texture_atlas;

#[allow(unused)]
const BYTES_CLICKER: &[u8] = include_bytes!(
    "../../../../adaptors/rust/slime-clicker/target/wasm32-wasip2/debug/slime_clicker.wasm"
);

#[allow(unused)]
const BYTES_DRAGGER: &[u8] = include_bytes!(
    // "../../../../adaptors/csharp/SlimeDragger/wasiconsole/bin/Debug/net10.0/wasi-wasm/AppBundle/dotnet.wasm"
    // "../../../../adaptors/csharp/SlimeDragger/bin/Debug/net10.0/wasiconsole.wasm",
    // "../../../../adaptors/csharp/SlimeDragger/bin/Release/net10.0/wasi-wasm/publish/wasiconsole.wasm",
    //"../../../../adaptors/csharp/SlimeDragger/bin/Release/net10.0/wasi-wasm/native/wasiconsole.wasm",
    "../../../../adaptors/csharp/SlimeDragger/bin/Debug/net10.0/wasi-wasm/publish/adder.wasm"
);

fn main() {
    std::thread::Builder::new()
        .stack_size(128 * 1024 * 1024)
        .spawn(|| {
            macroquad::Window::new("Liquislime", main_());
        })
        .unwrap()
        .join()
        .unwrap();
}

async fn main_() {
    setup::setup_panic_hook();

    let faction0 = Faction::new(0, liquislime_core::Color::new(255, 128, 0));
    let faction1 = Faction::new(1, liquislime_core::Color::new(0, 255, 64));

    let factions = vec![faction0, faction1];

    let screen = Screen::new(InputHelper::screen_size(), 0.075f32);
    let state = GameState::new(factions, 50, 50, screen);
    let input_query = Box::new(InputQueryImpl);
    let mut state = FullState::new(state, input_query);

    println!("loading plugin1");
    state.adaptors.push(Box::new(examples::PanCamera));
    state.adaptors.push(Box::new(
        liquislime_wasmi::WasmiAdaptor::new(BYTES_CLICKER).unwrap(),
    ));

    // println!("loading plugin2");
    // state.adaptors.push(Box::new(
    //     liquislime_wasmi::WasmiAdaptor::new(BYTES_DRAGGER).unwrap(),
    // ));

    println!("loading plugins done");

    state.game_state.grids.set_amount(
        state.game_state.factions[0].id(),
        TilePosition::new(2, 4),
        SlimeAmount::from_integer(50000),
    );

    state.game_state.grids.set_amount(
        state.game_state.factions[1].id(),
        TilePosition::new(8, 5),
        SlimeAmount::from_integer(60000),
    );

    loop {
        let time_passed = TimeInterval::from_seconds(get_frame_time() as f64);

        state.game_state.screen.size = InputHelper::screen_size();
        state.update(time_passed);
        update(&mut state.game_state);
        render::render_game(&state.game_state);

        // draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}

fn update(state: &mut GameState) {
    // state.update(TimeInterval::from_seconds(get_frame_time() as f64));
    input::process_inputs(state);
}
