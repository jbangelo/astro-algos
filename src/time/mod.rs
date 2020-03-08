//! Representations of time for use in astronomy.
//!
//! Time is a critical part of astronomy, as the location of everything in space changes with time.
//! There is a long history to how we represent time that spans many millennia and many cultures.
//! For our purposes in this crate we like to represent time as a "Julian Day". The Julian Day
//! representation is the number of days since Noon on Jan 1 in the year -4712 as a Real number.

use std::convert::From;

pub mod date;

/// Representation of a Julian Day
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct JD(f64);

impl JD {
    /// Converts a `JD` into a bare `f64`
    pub fn as_f64(&self) -> f64 {
        self.0
    }
}

impl From<f64> for JD {
    fn from(item: f64) -> Self {
        assert!(item >= 0.0, "Invalid JD value: {}", item);
        JD(item)
    }
}

impl From<JD> for f64 {
    fn from(item: JD) -> Self {
        item.0
    }
}
