use derive_more::{Add, AddAssign, Neg, Sub, SubAssign};
use std::ops::{Div, Mul};

const ONE_SLIME_AMOUNT: i64 = u32::MAX as i64;

#[derive(
    Debug,
    Clone,
    Copy,
    Default,
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
)]
pub struct SlimeAmount {
    pub amount: i64,
}

impl SlimeAmount {
    pub const fn new() -> Self {
        Self { amount: 0 }
    }

    pub const fn from_integer(amount: i64) -> Self {
        Self {
            amount: amount * ONE_SLIME_AMOUNT,
        }
    }

    pub fn from_float(amount: f64) -> Self {
        Self {
            amount: (amount * (ONE_SLIME_AMOUNT as f64)) as i64,
        }
    }

    pub fn non_negative(self) -> Self {
        Self {
            amount: self.amount.max(0),
        }
    }

    pub fn as_integer(self) -> i64 {
        self.amount / ONE_SLIME_AMOUNT
    }

    pub fn as_float(self) -> f64 {
        (self.amount as f64) / (ONE_SLIME_AMOUNT as f64)
    }
}

impl Mul<i64> for SlimeAmount {
    type Output = SlimeAmount;

    fn mul(self, rhs: i64) -> Self::Output {
        Self {
            amount: self.amount * rhs,
        }
    }
}

impl Div<i64> for SlimeAmount {
    type Output = SlimeAmount;

    fn div(self, rhs: i64) -> Self::Output {
        Self {
            amount: self.amount / rhs,
        }
    }
}

impl Mul<f64> for SlimeAmount {
    type Output = SlimeAmount;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            amount: (self.amount as f64 * rhs).round() as i64,
        }
    }
}

impl Div<f64> for SlimeAmount {
    type Output = SlimeAmount;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            amount: (self.amount as f64 / rhs).round() as i64,
        }
    }
}

impl<'de> serde::Deserialize<'de> for SlimeAmount {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // TODO: user error
        let text = <String as serde::Deserialize>::deserialize(deserializer)?;
        let amount = text.parse::<i64>().unwrap();
        Ok(Self { amount })
    }
}
