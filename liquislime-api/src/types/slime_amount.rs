use derive_more::{Add, AddAssign, Neg, Sub, SubAssign};
use fp_bindgen::prelude::Serializable;
use std::ops::{Div, Mul};

const ONE_SLIME_AMOUNT: i64 = u32::MAX as i64;

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
pub struct SlimeAmount(pub i64);
