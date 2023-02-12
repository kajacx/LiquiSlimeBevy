use fp_bindgen::prelude::Serializable;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serializable, Serialize, Deserialize,
)]
// TODO: this should be homogenous with keyboard / joystick buttons?
pub enum MouseButton {
    LeftButton,
    RightButton,
    MiddleButton,
    // TODO: add the 2 side buttons.
}
