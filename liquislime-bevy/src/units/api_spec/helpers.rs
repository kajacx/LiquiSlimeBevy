use std::ops::DerefMut;

use bevy::prelude::World;

use crate::components::SlimeGrid;

use super::types::*;

use super::super::{
    global_storage::{get_current_unit, get_world},
    UnitId,
};

pub fn get_slime_grid<'a>(world: &'a mut World) -> impl DerefMut<Target = SlimeGrid> + 'a {
    world
        .query::<&mut SlimeGrid>()
        .get_single_mut(world)
        .expect("Slime Grid should have been created")
}
