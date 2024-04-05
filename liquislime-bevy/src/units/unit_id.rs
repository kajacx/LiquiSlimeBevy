use bevy::prelude::Component;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Component)]
pub struct UnitId(pub u32);
