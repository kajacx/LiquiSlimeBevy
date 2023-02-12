use derive_more::{Add, AddAssign, Neg, Sub, SubAssign};
use fp_bindgen::prelude::Serializable;

#[derive(
    Debug,
    Clone,
    Copy,
    Add,
    Sub,
    Neg,
    AddAssign,
    SubAssign,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Default,
    Serializable,
)]
pub struct TimeInterval(i64);
