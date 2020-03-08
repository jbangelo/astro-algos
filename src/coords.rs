use crate::angle::Angle;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RaDec {
    pub ra: Angle,
    pub dec: Angle,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ecliptic {
    pub lng: Angle,
    pub lat: Angle,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AltAz {
    pub alt: Angle,
    pub az: Angle,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct LatLng {
    pub lat: Angle,
    pub lng: Angle,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct HeliocentricSpherical {
    pub latitude: Angle,
    pub longitude: Angle,
    pub radius: f64,
}
