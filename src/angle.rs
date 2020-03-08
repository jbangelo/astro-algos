#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Angle {
    value: f64,
}

impl Angle {
    pub fn from_degrees(d: f64) -> Angle {
        Angle {
            value: d.to_radians(),
        }
    }

    pub fn from_radians(r: f64) -> Angle {
        Angle { value: r }
    }

    pub fn from_dms(degrees: u8, mins: u8, secs: f64) -> Angle {
        let deg = (degrees as f64) + (mins as f64) / 60.0 + (secs / 3600.0);
        Angle {
            value: (deg % 360.0).to_radians(),
        }
    }

    pub fn from_hms(hours: u8, mins: u8, secs: f64) -> Angle {
        let deg = ((hours as f64) + (mins as f64 / 60.0) + (secs / 3600.0)) * (360.0 / 24.0);
        Angle {
            value: (deg % 360.0).to_radians(),
        }
    }

    pub fn to_radians(&self) -> f64 {
        self.value
    }

    pub fn to_degrees(&self) -> f64 {
        self.value.to_degrees()
    }

    pub fn sin(&self) -> f64 {
        self.value.sin()
    }
    pub fn cos(&self) -> f64 {
        self.value.cos()
    }
    pub fn tan(&self) -> f64 {
        self.value.tan()
    }

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

    pub fn asin(item: f64) -> Angle {
        Angle { value: item.asin() }
    }

    pub fn acos(item: f64) -> Angle {
        Angle { value: item.acos() }
    }

    pub fn atan2(num: f64, denom: f64) -> Angle {
        Angle {
            value: num.atan2(denom),
        }
    }

    pub fn atan(item: f64) -> Angle {
        Angle { value: item.atan() }
    }
}

impl std::ops::Add for Angle {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            value: self.value + rhs.value,
        }
    }
}

impl std::ops::Add for &Angle {
    type Output = Angle;

    fn add(self, rhs: Self) -> Angle {
        Angle {
            value: self.value + rhs.value,
        }
    }
}

impl std::ops::AddAssign for Angle {
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
    }
}

impl std::ops::Sub for Angle {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            value: self.value - rhs.value,
        }
    }
}

impl std::ops::Sub for &Angle {
    type Output = Angle;

    fn sub(self, rhs: Self) -> Angle {
        Angle {
            value: self.value - rhs.value,
        }
    }
}

impl std::ops::SubAssign for Angle {
    fn sub_assign(&mut self, rhs: Self) {
        self.value -= rhs.value;
    }
}

#[cfg(test)]
mod tests {
    use super::Angle;

    #[test]
    fn from_degrees() {
        assert_eq!(Angle::from_degrees(0.0), Angle { value: 0.0 });
        assert_eq!(
            Angle::from_degrees(1.0),
            Angle {
                value: 0.01745329251994329577
            }
        );
        assert_eq!(
            Angle::from_degrees(25.4345),
            Angle {
                value: 0.44391576859849775626
            }
        );
    }

    #[test]
    fn from_dms() {
        assert_eq!(Angle::from_dms(0, 0, 0.0), Angle::from_degrees(0.0));
        assert_eq!(Angle::from_dms(1, 0, 0.0), Angle::from_degrees(1.0));
        assert_eq!(
            Angle::from_dms(0, 1, 0.0),
            Angle::from_degrees(0.01666666666666666667)
        );
        assert_eq!(
            Angle::from_dms(0, 0, 1.0),
            Angle::from_degrees(0.00027777777777777778)
        );
        assert_eq!(
            Angle::from_dms(34, 55, 25.5436353),
            Angle::from_degrees(34.92376212091666666667)
        );
    }

    #[test]
    fn from_hms() {
        assert_eq!(Angle::from_hms(0, 0, 0.0), Angle::from_degrees(0.0));
        assert_eq!(Angle::from_hms(1, 0, 0.0), Angle::from_degrees(15.0));
        assert_eq!(Angle::from_hms(0, 1, 0.0), Angle::from_degrees(0.25));
        assert_eq!(
            Angle::from_hms(0, 0, 1.0),
            Angle::from_degrees(0.00416666666666666667)
        );
        assert_eq!(
            Angle::from_hms(14, 55, 25.5436353),
            Angle::from_degrees(223.85643181375)
        );
    }

    #[test]
    fn arcsin() {
        assert_eq!(Angle::asin(0.0), Angle::from_degrees(0.0));
        assert_eq!(Angle::asin(1.0), Angle::from_degrees(90.0));
        assert_eq!(Angle::asin(-1.0), Angle::from_degrees(-90.0));

        assert!(Angle::asin(1.001).to_radians().is_nan());
        assert!(Angle::asin(-1.001).to_radians().is_nan());
    }

    #[test]
    fn arccos() {
        assert_eq!(Angle::acos(0.0), Angle::from_degrees(90.0));
        assert_eq!(Angle::acos(1.0), Angle::from_degrees(0.0));
        assert_eq!(Angle::acos(-1.0), Angle::from_degrees(180.0));

        assert!(Angle::acos(1.001).to_radians().is_nan());
        assert!(Angle::acos(-1.001).to_radians().is_nan());
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
}
