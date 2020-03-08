//! This crate provides several implementations of the algorithms described in "Astronomical
//! Algorithms" by Jean Meeus (2nd edition, ISBN: 0-943396-61-1)
//!
//! Probably the most interesting algorithm is the planetary position calculator. There are also
//! utilities dealing with andles and time. The time representation is somewhat different than is
//! typical.

pub mod angle;
pub mod coords;
pub mod planets;
pub mod time;
