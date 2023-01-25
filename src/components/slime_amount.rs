use derive_more::{Add, AddAssign, From, Neg, Sub, SubAssign};
use std::ops::{Div, Mul};

use bevy::prelude::*;

#[derive(
    Component,
    Debug,
    Clone,
    Copy,
    Add,
    Sub,
    Neg,
    AddAssign,
    SubAssign,
    From,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
)]
pub struct SlimeAmount(pub i64);

impl SlimeAmount {
    pub fn new(amount: i64) -> Self {
        Self(amount)
    }

    pub fn non_negative(self) -> Self {
        Self(self.0.max(0))
    }
}

impl Mul<i64> for SlimeAmount {
    type Output = SlimeAmount;

    #[inline]
    fn mul(self, rhs: i64) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl Div<i64> for SlimeAmount {
    type Output = SlimeAmount;

    #[inline]
    fn div(self, rhs: i64) -> Self::Output {
        Self(self.0 / rhs)
    }
}
