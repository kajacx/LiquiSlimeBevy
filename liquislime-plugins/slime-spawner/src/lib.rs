use liquislime_api::*;
use std::sync::Mutex;

static SLIME_AMOUNT: Mutex<SlimeAmount> = Mutex::new(SlimeAmount::from_integer(0));

#[fp_export_impl(liquislime_api)]
fn update(time_elapsed: TimeInterval) {
    let amount_dbg =
        *SLIME_AMOUNT.lock().unwrap() + SlimeAmount::from_float(time_elapsed.to_seconds());
    *SLIME_AMOUNT.lock().unwrap() = amount_dbg;

    if was_mouse_just_pressed(MouseButton::LeftButton) {
        set_own_position(
            get_mouse_position()
                .expect("TODO: single method to return clicked and position")
                .to_tile_position(),
        );
    }

    let added_amount_per_second = SlimeAmount::from_integer(1000);

    if let Some(pos) = get_mouse_position() {
        let pos = pos.to_tile_position();
        //let amount =  added_amount_per_second * time_elapsed.to_seconds();
        let amount = amount_dbg * time_elapsed.to_seconds();
        add_slime_amount(pos, amount);
    }

    let amount = added_amount_per_second * time_elapsed.to_seconds();
    // add_slime_amount(get_own_position(), amount);
}
