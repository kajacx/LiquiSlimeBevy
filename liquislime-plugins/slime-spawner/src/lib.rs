use liquislime_api::*;

#[fp_export_impl(liquislime_api)]
fn update(time_elapsed: TimeInterval) {
    if was_mouse_just_pressed(MouseButton::LeftButton) {
        set_own_position(get_mouse_position().to_tile_position());
    }

    let amount = get_slime_amount(get_own_position());
    let amount = amount * time_elapsed.to_seconds();
    set_slime_amount(get_own_position(), amount);
}
