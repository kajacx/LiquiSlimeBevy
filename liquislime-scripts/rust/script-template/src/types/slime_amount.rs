use crate::api::{FromWasmAbi, ToWasmAbi};
use crate::DynValue;
use anyhow::Result;
use derive_more::{Add, AddAssign, Neg, Sub, SubAssign};
use std::fmt::Debug;

use std::ops::{Div, Mul};

const ONE_SLIME_AMOUNT: i64 = u32::MAX as i64;

#[derive(
    Clone, Copy, Default, Add, Sub, Neg, AddAssign, SubAssign, PartialEq, Eq, Hash, PartialOrd, Ord,
)]
pub struct SlimeAmount(i64);

impl SlimeAmount {
    pub const fn new() -> Self {
        Self(0)
    }

    pub const fn from_integer(amount: i64) -> Self {
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

    pub fn serialize(self, writer: &mut impl std::io::Write) -> Result<()> {
        Ok(rmp::encode::write_i64(writer, self.0)?)
    }

    pub fn deserialize(reader: &mut impl std::io::Read) -> Result<Self> {
        Ok(Self(rmp::decode::read_i64(reader)?))
    }
}

impl Mul<i64> for SlimeAmount {
    type Output = SlimeAmount;

    fn mul(self, rhs: i64) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl Div<i64> for SlimeAmount {
    type Output = SlimeAmount;

    fn div(self, rhs: i64) -> Self::Output {
        Self(self.0 / rhs)
    }
}

impl Mul<f64> for SlimeAmount {
    type Output = SlimeAmount;

    fn mul(self, rhs: f64) -> Self::Output {
        Self((self.0 as f64 * rhs).round() as i64)
    }
}

impl Div<f64> for SlimeAmount {
    type Output = SlimeAmount;

    fn div(self, rhs: f64) -> Self::Output {
        Self((self.0 as f64 / rhs).round() as i64)
    }
}

impl ToWasmAbi for SlimeAmount {
    type Abi = i64;

    fn to_wasm_abi(&self) -> Self::Abi {
        self.0
    }
}

impl FromWasmAbi for SlimeAmount {
    type Abi = i64;

    fn from_wasm_abi(abi: Self::Abi) -> Result<Self> {
        Ok(Self(abi))
    }
}

impl From<SlimeAmount> for DynValue {
    fn from(value: SlimeAmount) -> Self {
        Self::SlimeAmount(value)
    }
}

impl Debug for SlimeAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SlimeAmount({})", self.as_float())
    }
}