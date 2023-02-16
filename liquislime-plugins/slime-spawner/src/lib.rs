use liquislime_api::*;

#[fp_export_impl(liquislime_api)]
fn update(time_elapsed: TimeInterval) {
    if was_mouse_just_pressed(MouseButton::LeftButton) {
        set_own_position(
            get_mouse_position()
                .expect("TODO: single method to return clicked and position")
                .to_tile_position(),
        );
    }

    let added_amount_per_second = SlimeAmount::from_integer(1000);

    let amount = get_slime_amount(get_own_position());
    let amount = amount + added_amount_per_second * time_elapsed.to_seconds();
    set_slime_amount(get_own_position(), amount);
}
