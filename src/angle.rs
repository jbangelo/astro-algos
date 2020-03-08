//! Angles are a core part of astronomical measurements.
//!
//! They are used everywhere and can have several representations. This module helps to handle the
//! conversion between these representations into a common type.

/// The core representation of an angle. Internally we simply use a `f64` for the representation in
/// radians but that isn't directly accessible. Instead, you should use the provided conversion
/// functions.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Angle(f64);

impl Angle {
    /// Converts a bare `f64` into an `Angle`, treating the `f64` as if it were in units of degrees.
    pub fn from_degrees(d: f64) -> Angle {
        Angle(d.to_radians())
    }

    /// Converts a bare `f64` into an `Angle`, treating the `f64` as if it were in units of radians.
    pub fn from_radians(r: f64) -> Angle {
        Angle(r)
    }

    /// Converts an angle represented as degrees, minutes, and second into an `Angle`.
    pub fn from_dms(angle: DegreesMinutesSeconds) -> Angle {
        angle.as_angle()
    }

    /// Converts an angle represented as hours, minutes, and second into an `Angle`.
    pub fn from_hms(angle: HoursMinutesSeconds) -> Angle {
        angle.as_angle()
    }

    /// Converts an `Angle` into a bare `f64` that is in units of radians
    pub fn as_radians(&self) -> f64 {
        self.0
    }

    /// Converts an `Angle` into a bare `f64` that is in units of degrees
    pub fn as_degrees(&self) -> f64 {
        self.0.to_degrees()
    }

    /// Converts an `Angle` into a `DegreesMinutesSeconds`
    pub fn as_dms(&self) -> DegreesMinutesSeconds {
        DegreesMinutesSeconds::from_angle(self.clone())
    }

    /// Converts an `Angle` into a `HoursMinutesSeconds`
    pub fn as_hms(&self) -> HoursMinutesSeconds {
        HoursMinutesSeconds::from_angle(self.clone())
    }

    /// Gets the sine of the angle.
    pub fn sin(&self) -> f64 {
        self.0.sin()
    }

    /// Gets the cosine of the angle.
    pub fn cos(&self) -> f64 {
        self.0.cos()
    }

    /// Gets the tangent of the angle.
    pub fn tan(&self) -> f64 {
        self.0.tan()
    }

    /// Gets the arcsine angle of a value
    pub fn asin(item: f64) -> Angle {
        Angle(item.asin())
    }

    /// Gets the arccosine angle of a value
    pub fn acos(item: f64) -> Angle {
        Angle(item.acos())
    }

    /// Gets the arctangent angle of a value
    pub fn atan(item: f64) -> Angle {
        Angle(item.atan())
    }

    /// Gets the arctangent angle of a value, using the typical atan2 function
    pub fn atan2(num: f64, denom: f64) -> Angle {
        Angle(num.atan2(denom))
    }

    /// Wraps the value of an angle so that is is between the two given limits
    ///
    /// In certain circumstances it is customary to keep the value of an angle between certain
    /// values, but the limits of the values are dependent on the use case. For example longitude
    /// values are between -180 and 180 degrees, while latitude values are between -90 and 90
    /// degrees.
    /// # Examples
    /// ```
    /// use astro_algos::angle::Angle;
    /// let far_east = Angle::from_degrees(180.0);
    /// let far_west = Angle::from_degrees(-180.0);
    /// let longitude = Angle::from_degrees(190.0).wrap(&far_west, &far_east); // Makes `longitude` == -170 degrees
    /// ```
    pub fn wrap(mut self, low_limit: &Angle, high_limit: &Angle) -> Angle {
        assert!(high_limit > low_limit);
        let range = high_limit - low_limit;
        while self > *high_limit {
            self -= range;
        }

        while self < *low_limit {
            self += range;
        }

        self
    }
}

impl std::ops::Add for Angle {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl std::ops::Add for &Angle {
    type Output = Angle;

    fn add(self, rhs: Self) -> Angle {
        Angle(self.0 + rhs.0)
    }
}

impl std::ops::AddAssign for Angle {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl std::ops::Sub for Angle {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}

impl std::ops::Sub for &Angle {
    type Output = Angle;

    fn sub(self, rhs: Self) -> Angle {
        Angle(self.0 - rhs.0)
    }
}

impl std::ops::SubAssign for Angle {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct DegreesMinutesSeconds {
    pub degrees: i32,
    pub minutes: i32,
    pub seconds: f64,
}

impl DegreesMinutesSeconds {
    pub fn from_angle(angle: Angle) -> Self {
        let degrees = angle.as_degrees();
        let minutes = degrees.fract().abs() * 60.0;
        let seconds = minutes.fract().abs() * 60.0;

        Self {
            degrees: degrees.trunc() as i32,
            minutes: minutes.trunc() as i32,
            seconds,
        }
    }

    pub fn as_angle(&self) -> Angle {
        let deg = (self.degrees as f64) + (self.minutes as f64) / 60.0 + (self.seconds / 3600.0);
        Angle::from_degrees(deg)
    }
}

impl std::fmt::Display for DegreesMinutesSeconds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}°{}′{:.3}″", self.degrees, self.minutes, self.seconds)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct HoursMinutesSeconds {
    pub hours: i32,
    pub minutes: i32,
    pub seconds: f64,
}

impl HoursMinutesSeconds {
    pub fn from_angle(angle: Angle) -> Self {
        let hours = angle.as_degrees() / 15.0;
        let minutes = hours.fract().abs() * 60.0;
        let seconds = minutes.fract().abs() * 60.0;

        Self {
            hours: hours.trunc() as i32,
            minutes: minutes.trunc() as i32,
            seconds,
        }
    }

    pub fn as_angle(&self) -> Angle {
        let deg =
            ((self.hours as f64) + (self.minutes as f64) / 60.0 + (self.seconds / 3600.0)) * 15.0;
        Angle::from_degrees(deg)
    }
}

impl std::fmt::Display for HoursMinutesSeconds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}ʰ{}ᵐ{:.3}ˢ", self.hours, self.minutes, self.seconds)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn from_degrees() {
        assert_eq!(Angle::from_degrees(0.0), Angle(0.0));
        assert_eq!(Angle::from_degrees(1.0), Angle(0.01745329251994329577));
        assert_eq!(Angle::from_degrees(25.4345), Angle(0.44391576859849775626));
    }

    #[test]
    fn from_dms() {
        assert_eq!(
            Angle::from_dms(DegreesMinutesSeconds {
                degrees: 0,
                minutes: 0,
                seconds: 0.0
            }),
            Angle::from_degrees(0.0)
        );
        assert_eq!(
            Angle::from_dms(DegreesMinutesSeconds {
                degrees: 1,
                minutes: 0,
                seconds: 0.0
            }),
            Angle::from_degrees(1.0)
        );
        assert_eq!(
            Angle::from_dms(DegreesMinutesSeconds {
                degrees: 0,
                minutes: 1,
                seconds: 0.0
            }),
            Angle::from_degrees(0.01666666666666666667)
        );
        assert_eq!(
            Angle::from_dms(DegreesMinutesSeconds {
                degrees: 0,
                minutes: 0,
                seconds: 1.0
            }),
            Angle::from_degrees(0.00027777777777777778)
        );
        assert_eq!(
            Angle::from_dms(DegreesMinutesSeconds {
                degrees: 34,
                minutes: 55,
                seconds: 25.5436353
            }),
            Angle::from_degrees(34.92376212091666666667)
        );
    }

    #[test]
    fn arcsin() {
        assert_eq!(Angle::asin(0.0), Angle::from_degrees(0.0));
        assert_eq!(Angle::asin(1.0), Angle::from_degrees(90.0));
        assert_eq!(Angle::asin(-1.0), Angle::from_degrees(-90.0));

        assert!(Angle::asin(1.001).as_radians().is_nan());
        assert!(Angle::asin(-1.001).as_radians().is_nan());
    }

    #[test]
    fn arccos() {
        assert_eq!(Angle::acos(0.0), Angle::from_degrees(90.0));
        assert_eq!(Angle::acos(1.0), Angle::from_degrees(0.0));
        assert_eq!(Angle::acos(-1.0), Angle::from_degrees(180.0));

        assert!(Angle::acos(1.001).as_radians().is_nan());
        assert!(Angle::acos(-1.001).as_radians().is_nan());
    }

    #[test]
    fn arctan() {
        assert_eq!(Angle::atan(0.0), Angle::from_degrees(0.0));
        assert_eq!(Angle::atan(1.0), Angle::from_degrees(45.0));
        assert_eq!(Angle::atan(-1.0), Angle::from_degrees(-45.0));

        assert_eq!(Angle::atan(std::f64::MAX), Angle::from_degrees(90.0));
        assert_eq!(Angle::atan(std::f64::MIN), Angle::from_degrees(-90.0));
    }

    #[test]
    fn arctan2() {
        assert_eq!(Angle::atan2(0.0, 0.0), Angle::from_degrees(0.0));
        assert_eq!(Angle::atan2(0.0, 1.0), Angle::from_degrees(0.0));
        assert_eq!(Angle::atan2(1.0, 0.0), Angle::from_degrees(90.0));
        assert_eq!(Angle::atan2(1.0, 1.0), Angle::from_degrees(45.0));
        assert_eq!(Angle::atan2(0.0, -1.0), Angle::from_degrees(180.0));
        assert_eq!(Angle::atan2(-1.0, 0.0), Angle::from_degrees(-90.0));
        assert_eq!(Angle::atan2(-1.0, -1.0), Angle::from_degrees(-135.0));

        assert_eq!(
            Angle::atan2(std::f64::MAX, std::f64::MAX),
            Angle::from_degrees(45.0)
        );
    }

    #[test]
    fn dms_conversions() {
        assert_eq!(
            DegreesMinutesSeconds {
                degrees: 0,
                minutes: 0,
                seconds: 0.0
            }
            .as_angle(),
            Angle::from_degrees(0.0)
        );
        assert_eq!(
            DegreesMinutesSeconds {
                degrees: 1,
                minutes: 0,
                seconds: 0.0
            }
            .as_angle(),
            Angle::from_degrees(1.0)
        );
        assert_eq!(
            DegreesMinutesSeconds {
                degrees: 0,
                minutes: 1,
                seconds: 0.0
            },
            DegreesMinutesSeconds::from_angle(Angle::from_degrees(0.01666666666666666667))
        );
        assert_eq!(
            DegreesMinutesSeconds {
                degrees: 0,
                minutes: 0,
                seconds: 1.0
            },
            DegreesMinutesSeconds::from_angle(Angle::from_degrees(0.00027777777777777778))
        );
        assert_eq!(
            DegreesMinutesSeconds {
                degrees: 34,
                minutes: 55,
                seconds: 25.543635299987955
            },
            Angle::from_degrees(34.92376212091666666667).as_dms()
        );
    }

    #[test]
    fn hms_conversions() {
        assert_approx_eq!(
            HoursMinutesSeconds {
                hours: 7,
                minutes: 45,
                seconds: 18.946
            }
            .as_angle()
            .as_degrees(),
            116.328_942
        );
        let angle = HoursMinutesSeconds::from_angle(Angle::from_degrees(-295.647_867));
        let hms = HoursMinutesSeconds {
            hours: -19,
            minutes: 42,
            seconds: 35.488,
        };
        assert_eq!(angle.hours, hms.hours);
        assert_eq!(angle.minutes, hms.minutes);
        assert_approx_eq!(angle.seconds, hms.seconds, 1e-4);
    }
}
