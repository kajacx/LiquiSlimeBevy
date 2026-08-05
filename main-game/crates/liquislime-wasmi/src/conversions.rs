use crate::bindings;

impl From<liquislime_core::Position> for bindings::Position {
    fn from(value: liquislime_core::Position) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

impl From<bindings::Position> for liquislime_core::Position {
    fn from(value: bindings::Position) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}
