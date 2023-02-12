use fp_bindgen::prelude::Serializable;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serializable)]
// TODO: this should be homogenous with keyboard / joystick buttons?
pub enum MouseButton {
    LeftButton,
    RightButton,
    MiddleButton,
    // TODO: add the 2 side buttons.
}
