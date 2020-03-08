//! Coordinates are how you represent a location in a reference frame.
//!
//! This module implements several coordinate types that are commonly used in astronomy.
use crate::angle::Angle;

/// Spherical coordinates centered on the sun, relative to the J2000.0 epoch.
/// The radius is in units of astronomical units (i.e. 149597870700 meters)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct HeliocentricSpherical {
    pub latitude: Angle,
    pub longitude: Angle,
    pub radius: f64,
}
