use std::ops::DerefMut;

use bevy::prelude::World;

use crate::components::SlimeGrid;

pub fn get_slime_grid<'a>(world: &'a mut World) -> impl DerefMut<Target = SlimeGrid> + 'a {
    world
        .query::<&mut SlimeGrid>()
        .get_single_mut(world)
        .expect("Slime Grid should have been created")
}
