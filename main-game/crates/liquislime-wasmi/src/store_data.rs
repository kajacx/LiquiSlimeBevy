use std::ptr::NonNull;

use liquislime_core::GameInteraction;
use wasmi_component::HostResult;

pub(crate) struct StoreData {
    game_interaction: Option<NonNull<GameInteraction<'static>>>,
}

impl StoreData {
    pub fn new() -> Self {
        Self {
            game_interaction: None,
        }
    }

    pub(crate) fn set_interaction(&mut self, game_interaction: &mut GameInteraction) {
        unsafe {
            let game_interaction = NonNull::from_mut(game_interaction.with_static_lifetime());
            self.game_interaction = Some(game_interaction);
        }
    }

    fn game_interaction(&mut self) -> &mut GameInteraction<'_> {
        unsafe { self.game_interaction.unwrap().as_mut() }
    }

    pub(crate) fn clear_interaction(&mut self) {
        self.game_interaction = None;
    }
}

impl crate::bindings::AdaptorImports for StoreData {
    fn get_mouse_world_position(&mut self) -> HostResult<crate::bindings::Position> {
        Ok(self.game_interaction().get_mouse_world_position().into())
    }
}
