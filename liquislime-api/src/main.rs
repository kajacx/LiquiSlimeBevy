mod types;
use types::*;

fp_import! {
    fn level_width() -> i32;
    fn level_height() -> i32;

    fn get_own_position() -> TilePosition;
    fn set_own_position(position: TilePosition);

    fn get_slime_amount(position: TilePosition) -> SlimeAmount;
    fn set_slime_amount(position: TilePosition, SlimeAmount);
}

fn_export! {
    fn init();

    fn on_mouse_click(event: MouseEvent);

    fn update(elapsed: TimeInterval);
    fn render();
}

fn main() {
    println!("Hello, world!");
}
