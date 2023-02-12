use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "api-generation", derive(fp_bindgen::prelude::Serializable))]
// TODO: this should be homogenous with keyboard / joystick buttons?
pub enum MouseButton {
    LeftButton,
    RightButton,
    MiddleButton,
    // TODO: add the 2 side buttons.
}
