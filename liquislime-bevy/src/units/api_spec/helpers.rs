use std::{borrow::Borrow, ops::DerefMut};

use bevy::prelude::*;

use crate::{components::SlimeGrid, units::global_storage::get_world};

pub fn get_slime_grid<'a>(world: &'a mut World) -> impl DerefMut<Target = SlimeGrid> + 'a {
    world
        .query::<&mut SlimeGrid>()
        .get_single_mut(world)
        .expect("Slime Grid should have been created")
}

// pub fn api_mouse_button_to_bevy(
//     mouse_button: super::types::MouseButton,
// ) -> bevy::prelude::MouseButton {
//     match mouse_button {
//         super::MouseButton::LeftButton => bevy::prelude::MouseButton::Left,
//         super::MouseButton::RightButton => bevy::prelude::MouseButton::Right,
//         super::MouseButton::MiddleButton => bevy::prelude::MouseButton::Middle,
//     }
// }

// pub fn read_mouse_input<T>(reader: impl FnOnce(&Input<MouseButton>) -> T) -> T {
//     let world = get_world();

//     let input = world
//         .get_resource::<Input<MouseButton>>()
//         .expect("Mouse input resource should exist");

//     reader(input.borrow())
// }
