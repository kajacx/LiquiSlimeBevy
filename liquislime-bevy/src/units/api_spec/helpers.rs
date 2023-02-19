use std::ops::DerefMut;

use bevy::prelude::World;

use crate::components::SlimeGrid;

use super::types::MouseButton;

pub fn get_slime_grid<'a>(world: &'a mut World) -> impl DerefMut<Target = SlimeGrid> + 'a {
    world
        .query::<&mut SlimeGrid>()
        .get_single_mut(world)
        .expect("Slime Grid should have been created")
}

pub fn api_mouse_button_to_bevy(mouse_button: MouseButton) -> bevy::prelude::MouseButton {
    match mouse_button {
        MouseButton::LeftButton => bevy::prelude::MouseButton::Left,
        MouseButton::RightButton => bevy::prelude::MouseButton::Right,
        MouseButton::MiddleButton => bevy::prelude::MouseButton::Middle,
    }
}
