//! Coordinates are how you represent a location in a reference frame.
//!
//! This module implements several coordinate types that are commonly used in astronomy.
use crate::angle::Angle;

pub trait Equinox {
    const OBLIQUITY: f64;
}

pub struct J2000 {}
impl Equinox for J2000 {
    const OBLIQUITY: f64 = 0.40909280402840346503;
}

pub struct B1950 {}
impl Equinox for B1950 {
    const OBLIQUITY: f64 = 0.40920621203253955258;
}

/// Spherical coordinates centered on the sun, relative to the J2000.0 epoch.
/// The radius is in units of astronomical units (i.e. 149597870700 meters)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct HeliocentricSpherical {
    pub latitude: Angle,
    pub longitude: Angle,
    pub radius: f64,
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Equatorial<E: Equinox> {
    pub right_ascention: Angle,
    pub declination: Angle,
    phantom: std::marker::PhantomData<E>,
}

impl<E> Equatorial<E>
where
    E: Equinox,
{
    pub fn to_ecliptical(&self) -> Ecliptical<E> {
        Ecliptical {
            longitude: Angle::atan2(
                self.right_ascention.sin() * E::OBLIQUITY.cos()
                    + self.declination.tan() * E::OBLIQUITY.sin(),
                self.right_ascention.cos(),
            ),
            latitude: Angle::asin(
                self.declination.sin() * E::OBLIQUITY.cos()
                    - self.declination.cos() * E::OBLIQUITY.sin() * self.right_ascention.sin(),
            ),
            phantom: std::marker::PhantomData,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Ecliptical<E: Equinox> {
    pub longitude: Angle,
    pub latitude: Angle,
    phantom: std::marker::PhantomData<E>,
}

impl<E> Ecliptical<E>
where
    E: Equinox,
{
    pub fn to_equatorial(&self) -> Equatorial<E> {
        Equatorial {
            right_ascention: Angle::atan2(
                self.longitude.sin() * E::OBLIQUITY.cos()
                    - self.latitude.tan() * E::OBLIQUITY.sin(),
                self.longitude.cos(),
            ),
            declination: Angle::asin(
                self.latitude.sin() * E::OBLIQUITY.cos()
                    + self.latitude.cos() * E::OBLIQUITY.sin() * self.longitude.sin(),
            ),
            phantom: std::marker::PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::angle::{DegreesMinutesSeconds, HoursMinutesSeconds};
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn ecliptical_to_equatorial() {
        // Example 13.a, page 95
        let ecliptical = Equatorial::<J2000> {
            right_ascention: HoursMinutesSeconds {
                hours: 7,
                minutes: 45,
                seconds: 18.946,
            }
            .as_angle(),
            declination: DegreesMinutesSeconds {
                degrees: 28,
                minutes: 1,
                seconds: 34.26,
            }
            .as_angle(),
            phantom: std::marker::PhantomData,
        }
        .to_ecliptical();
        assert_approx_eq!(ecliptical.longitude.as_degrees(), 113.215_630);
        assert_approx_eq!(ecliptical.latitude.as_degrees(), 6.684170);
    }
}
