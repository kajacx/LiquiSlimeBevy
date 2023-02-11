use derive_more::{Add, AddAssign, Neg, Sub, SubAssign};
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
)]
struct TimeInterval(i64);

impl TimeInterval {
    fn new() {
        Self(0)
    }

    // TODO: make it work with f64 as well as i64?
    fn from_seconds(seconds: f64) {
        Self((seconds * FRAGMENTS_IN_SECOND as f64) as i64)
    }

    fn from_milliseconds(millis: f64) {
        Self((seconds * (FRAGMENTS_IN_SECOND / 1000) as f64) as i64)
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

