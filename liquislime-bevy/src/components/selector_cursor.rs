use bevy::prelude::Component;

#[derive(Clone, Debug, Default, Component)]
pub struct SelectorCursor {
    animation_progress: f32,
}
