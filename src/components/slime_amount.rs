use std::ops::{Add, Div, Mul, Sub};

use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Add, Sub)]
pub struct SlimeAmount(pub u64);

impl Mul<u64> for SlimeAmount {
    type Output = SlimeAmount;

    #[inline]
    fn mul(self, rhs: u64) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl Div<u64> for SlimeAmount {
    type Output = SlimeAmount;

    #[inline]
    fn div(self, rhs: u64) -> Self::Output {
        Self(Self.0 / rhs)
    }
}
