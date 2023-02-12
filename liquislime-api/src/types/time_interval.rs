use derive_more::{Add, AddAssign, Neg, Sub, SubAssign};
use fp_bindgen::prelude::Serializable;
use serde::{Deserialize, Serialize};
use std::ops::{Div, Mul};

const FRAGMENTS_IN_SECOND: i64 = 18_000;

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
    Serialize,
    Deserialize,
)]
pub struct TimeInterval(i64);

impl TimeInterval {
    fn new() -> Self {
        Self(0)
    }

    // TODO: make it work with f64 as well as i64?
    fn from_seconds(seconds: f64) -> Self {
        Self((seconds * FRAGMENTS_IN_SECOND as f64) as i64)
    }

    fn from_milliseconds(milliseconds: f64) -> Self {
        Self((milliseconds * (FRAGMENTS_IN_SECOND / 1000) as f64) as i64)
    }
}

impl Mul<i64> for TimeInterval {
    type Output = TimeInterval;

    #[inline]
    fn mul(self, rhs: i64) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl Div<i64> for TimeInterval {
    type Output = TimeInterval;

    #[inline]
    fn div(self, rhs: i64) -> Self::Output {
        Self(self.0 / rhs)
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
    }
}
