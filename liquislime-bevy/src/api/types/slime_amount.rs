use derive_more::{Add, AddAssign, Neg, Sub, SubAssign};
use std::{
    fmt::Debug,
    ops::{Div, Mul},
};

const ONE_SLIME_AMOUNT: i64 = u32::MAX as i64;

#[derive(
    Clone, Copy, Default, Add, Sub, Neg, AddAssign, SubAssign, PartialEq, Eq, Hash, PartialOrd, Ord,
)]
pub struct ApiSlimeAmount(pub i64);

impl ApiSlimeAmount {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn from_integer(amount: i64) -> Self {
        Self(amount * ONE_SLIME_AMOUNT)
    }

    pub fn from_float(amount: f64) -> Self {
        Self((amount * (ONE_SLIME_AMOUNT as f64)) as i64)
    }

    pub fn non_negative(self) -> Self {
        Self(self.0.max(0))
    }

    pub fn as_integer(self) -> i64 {
        self.0 / ONE_SLIME_AMOUNT
    }

    pub fn as_float(self) -> f64 {
        (self.0 as f64) / (ONE_SLIME_AMOUNT as f64)
    }
}

impl Mul<i64> for ApiSlimeAmount {
    type Output = ApiSlimeAmount;

    fn mul(self, rhs: i64) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl Div<i64> for ApiSlimeAmount {
    type Output = ApiSlimeAmount;

    fn div(self, rhs: i64) -> Self::Output {
        Self(self.0 / rhs)
    }
}

impl Mul<f64> for ApiSlimeAmount {
    type Output = ApiSlimeAmount;

    fn mul(self, rhs: f64) -> Self::Output {
        Self((self.0 as f64 * rhs).round() as i64)
    }
}

impl Div<f64> for ApiSlimeAmount {
    type Output = ApiSlimeAmount;

    fn div(self, rhs: f64) -> Self::Output {
        Self((self.0 as f64 / rhs).round() as i64)
    }
}

impl Debug for ApiSlimeAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ApiSlimeAmount({})", self.as_float())
    }
}
