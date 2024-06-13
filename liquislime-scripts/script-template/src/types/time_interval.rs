use derive_more::{Add, AddAssign, Neg, Sub, SubAssign};
use std::ops::{Div, Mul};

use crate::api::{FromWasmAbi, ToWasmAbi};

const FRAGMENTS_IN_SECOND: i64 = 18_000;

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
pub struct TimeInterval {
    fragments: i64,
}

impl TimeInterval {
    pub const fn new() -> Self {
        Self { fragments: 0 }
    }

    pub fn from_seconds(seconds: f64) -> Self {
        Self {
            fragments: (seconds * FRAGMENTS_IN_SECOND as f64).round() as i64,
        }
    }

    pub fn from_milliseconds(milliseconds: f64) -> Self {
        Self {
            fragments: (milliseconds * (FRAGMENTS_IN_SECOND / 1000) as f64).round() as i64,
        }
    }

    pub fn to_seconds(self) -> f64 {
        (self.fragments as f64) / (FRAGMENTS_IN_SECOND as f64)
    }

    pub fn to_milliseconds(self) -> f64 {
        (self.fragments as f64) / ((FRAGMENTS_IN_SECOND * 1000) as f64)
    }
}

impl Mul<i64> for TimeInterval {
    type Output = TimeInterval;

    fn mul(self, rhs: i64) -> Self::Output {
        Self {
            fragments: self.fragments * rhs,
        }
    }
}

impl Div<i64> for TimeInterval {
    type Output = TimeInterval;

    fn div(self, rhs: i64) -> Self::Output {
        Self {
            fragments: self.fragments / rhs,
        }
    }
}

impl ToWasmAbi for TimeInterval {
    type Abi = i64;

    fn to_wasm_abi(&self) -> Self::Abi {
        self.fragments
    }
}

impl FromWasmAbi for TimeInterval {
    type Abi = i64;

    fn from_wasm_abi(abi: Self::Abi) -> Self {
        Self { fragments: abi }
    }
}

#[cfg(test)]
mod test {
    use super::TimeInterval;

    fn check_fragments_divide(amount: i64) {
        let second = TimeInterval::from_seconds(1.0);
        let divided = second / amount;
        assert_eq!(divided * amount, second);
    }

    #[test]
    fn divides_common_values() {
        // Common frame rates
        check_fragments_divide(24);
        check_fragments_divide(25);
        check_fragments_divide(30);
        check_fragments_divide(48);
        check_fragments_divide(50);
        check_fragments_divide(60);
        check_fragments_divide(75);
        check_fragments_divide(144);
        check_fragments_divide(240);

        // Make sure it can represent 1 millisecond accurately as well
        check_fragments_divide(1000);
        assert_eq!(
            TimeInterval::from_seconds(1.0),
            TimeInterval::from_milliseconds(1000.0)
        );
    }
}