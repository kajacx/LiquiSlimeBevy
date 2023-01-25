use derive_more::{Add, AddAssign, Neg, Sub, SubAssign};
use std::ops::{Div, Mul};

use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Add, Sub, Neg, AddAssign, SubAssign)]
pub struct SlimeAmount(pub i64);

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
