mod bindings {
    wit_bindgen::generate!({
        path: "../../../main-game/crates/liquislime-wasmi/liquislime-api.wit",
        // with: {
        //     "wasmi-component:component-examples/round-trip@0.1.0/person": crate::Person,
        //     "wasmi-component:component-examples/round-trip@0.1.0/data": crate::Data
        // }
    });

    use super::ClickerAdaptor;
    export!(ClickerAdaptor);
}

use bindings::liquislime::api::host_functions;

struct ClickerAdaptor;

impl bindings::Guest for ClickerAdaptor {
    fn update(_time_elapsed: f64) -> () {
        let position = host_functions::get_mouse_world_position();

        println!("mouse is at {}, {}", position.x, position.y);
    }
}
