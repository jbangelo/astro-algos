//! This module contains algorithms dealing with planets in our solar system
mod earth;
mod jupiter;
mod mars;
mod mercury;
mod neptune;
mod saturn;
mod uranus;
mod venus;

use crate::angle::Angle;
use crate::coords::HeliocentricSpherical;
use crate::time::JD;

/// Representation of the planets in our solar system.
pub enum Planet {
    Mercury,
    Venus,
    Earth,
    Mars,
    Jupiter,
    Saturn,
    Uranus,
    Neptune,
}

impl Planet {
    /// Computes the position of the planet at a given moment in time, for the J2000.0 equinox
    ///
    /// This function uses the VSOP-87B model of the planetary motions. As such it should be
    /// reasonably accurate for times within a couple of millenia of the year 2000. Beyond that the
    /// accuracy for Jupiter and Saturn start to degrade. Beyond +/- 4000 years from the year 2000
    /// the accuracy of the positions for the inner four planets degrade. Finally past +/- 6000 years
    /// from the year 2000 the accuracy of Uranus and Neptune's positions start to degrade.
    pub fn get_location(&self, t: &JD) -> HeliocentricSpherical {
        let tau = (t.to_f64() - 2451_545.0) / 365_250.0;
        let (l_terms, b_terms, r_terms) = match self {
            Planet::Mercury => (mercury::LTERMS, mercury::BTERMS, mercury::RTERMS),
            Planet::Venus => (venus::LTERMS, venus::BTERMS, venus::RTERMS),
            Planet::Earth => (earth::LTERMS, earth::BTERMS, earth::RTERMS),
            Planet::Mars => (mars::LTERMS, mars::BTERMS, mars::RTERMS),
            Planet::Jupiter => (jupiter::LTERMS, jupiter::BTERMS, jupiter::RTERMS),
            Planet::Saturn => (saturn::LTERMS, saturn::BTERMS, saturn::RTERMS),
            Planet::Uranus => (uranus::LTERMS, uranus::BTERMS, uranus::RTERMS),
            Planet::Neptune => (neptune::LTERMS, neptune::BTERMS, neptune::RTERMS),
        };

        let l = sum_terms(&l_terms, tau);
        let b = sum_terms(&b_terms, tau);
        let r = sum_terms(&r_terms, tau);

        HeliocentricSpherical {
            longitude: Angle::from_radians(l)
                .wrap(&Angle::from_degrees(0.0), &Angle::from_degrees(360.0)),
            latitude: Angle::from_radians(b)
                .wrap(&Angle::from_degrees(-90.0), &Angle::from_degrees(90.0)),
            radius: r,
        }
    }
}

fn sum_terms(terms: &[&[(f64, f64, f64)]], tau: f64) -> f64 {
    terms
        .iter()
        .zip(0..6)
        .map(|(power_terms, power)| {
            power_terms
                .iter()
                .map(|(a, b, c)| a * (b + c * tau).cos() * tau.powi(power as i32))
                .sum::<f64>()
        })
        .sum::<f64>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn mercury_position() {
        let position = Planet::Mercury.get_location(&JD::from(2451545.0));
        assert_approx_eq!(position.longitude.to_radians(), 4.4293481043, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), -0.0527573411, 1e-9);
        assert_approx_eq!(position.radius, 0.4664714751, 1e-9);
        let position = Planet::Mercury.get_location(&JD::from(2415020.0));
        assert_approx_eq!(position.longitude.to_radians(), 3.5095041512, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), 0.0564907883, 1e-9);
        assert_approx_eq!(position.radius, 0.4183426276, 1e-9);
        let position = Planet::Mercury.get_location(&JD::from(2378495.0));
        assert_approx_eq!(position.longitude.to_radians(), 2.1225631484, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), 0.1171864614, 1e-9);
        assert_approx_eq!(position.radius, 0.3233909531, 1e-9);
        let position = Planet::Mercury.get_location(&JD::from(2341970.0));
        assert_approx_eq!(position.longitude.to_radians(), 0.2641557554, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), -0.0680150537, 1e-9);
        assert_approx_eq!(position.radius, 0.3381563139, 1e-9);
        let position = Planet::Mercury.get_location(&JD::from(2305445.0));
        assert_approx_eq!(position.longitude.to_radians(), 5.2811474961, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), -0.1178238226, 1e-9);
        assert_approx_eq!(position.radius, 0.4326517759, 1e-9);
        let position = Planet::Mercury.get_location(&JD::from(2268920.0));
        assert_approx_eq!(position.longitude.to_radians(), 4.3854123464, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), -0.0468100865, 1e-9);
        assert_approx_eq!(position.radius, 0.4661523937, 1e-9);
        let position = Planet::Mercury.get_location(&JD::from(2232395.0));
        assert_approx_eq!(position.longitude.to_radians(), 3.4577380614, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), 0.0634626330, 1e-9);
        assert_approx_eq!(position.radius, 0.4152385207, 1e-9);
        let position = Planet::Mercury.get_location(&JD::from(2195870.0));
        assert_approx_eq!(position.longitude.to_radians(), 2.0443901595, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), 0.1140574170, 1e-9);
        assert_approx_eq!(position.radius, 0.3209366231, 1e-9);
        let position = Planet::Mercury.get_location(&JD::from(2159345.0));
        assert_approx_eq!(position.longitude.to_radians(), 0.1936433214, 1e-8);
        assert_approx_eq!(position.latitude.to_radians(), -0.0764174092, 1e-9);
        assert_approx_eq!(position.radius, 0.3414354247, 1e-9);
        let position = Planet::Mercury.get_location(&JD::from(2122820.0));
        assert_approx_eq!(position.longitude.to_radians(), 5.2319689071, 1e-8);
        assert_approx_eq!(position.latitude.to_radians(), -0.1160635432, 1e-9);
        assert_approx_eq!(position.radius, 0.4352063235, 1e-9);
    }

    #[test]
    fn venus_position() {
        let position = Planet::Venus.get_location(&JD::from(2451545.0));
        assert_approx_eq!(position.longitude.to_radians(), 3.1870221910, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), 0.0569782849, 1e-9);
        assert_approx_eq!(position.radius, 0.7202129248, 1e-9);
        let position = Planet::Venus.get_location(&JD::from(2415020.0));
        assert_approx_eq!(position.longitude.to_radians(), 5.9993518124, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), -0.0591709804, 1e-9);
        assert_approx_eq!(position.radius, 0.7274719352, 1e-9);
        let position = Planet::Venus.get_location(&JD::from(2378495.0));
        assert_approx_eq!(position.longitude.to_radians(), 2.5571297503, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), 0.0554510891, 1e-9);
        assert_approx_eq!(position.radius, 0.7185473293, 1e-9);
        let position = Planet::Venus.get_location(&JD::from(2341970.0));
        assert_approx_eq!(position.longitude.to_radians(), 5.3846889524, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), -0.0460995953, 1e-9);
        assert_approx_eq!(position.radius, 0.7283407523, 1e-9);
        let position = Planet::Venus.get_location(&JD::from(2305445.0));
        assert_approx_eq!(position.longitude.to_radians(), 1.9265887457, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), 0.0319707652, 1e-9);
        assert_approx_eq!(position.radius, 0.7186375045, 1e-9);
        let position = Planet::Venus.get_location(&JD::from(2268920.0));
        assert_approx_eq!(position.longitude.to_radians(), 4.7713211615, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), -0.0156777292, 1e-9);
        assert_approx_eq!(position.radius, 0.7273363751, 1e-9);
        let position = Planet::Venus.get_location(&JD::from(2232395.0));
        assert_approx_eq!(position.longitude.to_radians(), 1.2988483958, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), -0.0040667685, 1e-9);
        assert_approx_eq!(position.radius, 0.7205428515, 1e-9);
        let position = Planet::Venus.get_location(&JD::from(2195870.0));
        assert_approx_eq!(position.longitude.to_radians(), 4.1554559280, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), 0.0208254122, 1e-9);
        assert_approx_eq!(position.radius, 0.7247441174, 1e-9);
        let position = Planet::Venus.get_location(&JD::from(2159345.0));
        assert_approx_eq!(position.longitude.to_radians(), 0.6752327774, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), -0.0383268978, 1e-9);
        assert_approx_eq!(position.radius, 0.7235430454, 1e-9);
        let position = Planet::Venus.get_location(&JD::from(2122820.0));
        assert_approx_eq!(position.longitude.to_radians(), 3.5336333775, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), 0.0496161272, 1e-9);
        assert_approx_eq!(position.radius, 0.7215819773, 1e-9);
    }

    #[test]
    fn earth_position() {
        let position = Planet::Earth.get_location(&JD::from(2451545.0));
        assert_approx_eq!(position.longitude.to_radians(), 1.7519238637, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), -0.0000039656, 1e-9);
        assert_approx_eq!(position.radius, 0.9833276823, 1e-9);
        let position = Planet::Earth.get_location(&JD::from(2415020.0));
        assert_approx_eq!(position.longitude.to_radians(), 1.7634989198, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), 0.0002186910, 1e-9);
        assert_approx_eq!(position.radius, 0.9832689762, 1e-9);
        let position = Planet::Earth.get_location(&JD::from(2378495.0));
        assert_approx_eq!(position.longitude.to_radians(), 1.7750058558, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), 0.0004381095, 1e-9);
        assert_approx_eq!(position.radius, 0.9832274335, 1e-9);
        let position = Planet::Earth.get_location(&JD::from(2341970.0));
        assert_approx_eq!(position.longitude.to_radians(), 1.7865387214, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), 0.0006583865, 1e-9);
        assert_approx_eq!(position.radius, 0.9831498445, 1e-9);
        let position = Planet::Earth.get_location(&JD::from(2305445.0));
        assert_approx_eq!(position.longitude.to_radians(), 1.7980474965, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), 0.0008715329, 1e-9);
        assert_approx_eq!(position.radius, 0.9831254370, 1e-9);
        let position = Planet::Earth.get_location(&JD::from(2268920.0));
        assert_approx_eq!(position.longitude.to_radians(), 1.8095367659, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), 0.0010876906, 1e-9);
        assert_approx_eq!(position.radius, 0.9830816762, 1e-9);
        let position = Planet::Earth.get_location(&JD::from(2232395.0));
        assert_approx_eq!(position.longitude.to_radians(), 1.8211080285, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), 0.0013092603, 1e-9);
        assert_approx_eq!(position.radius, 0.9830754398, 1e-9);
        let position = Planet::Earth.get_location(&JD::from(2195870.0));
        assert_approx_eq!(position.longitude.to_radians(), 1.8326137391, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), 0.0015219401, 1e-9);
        assert_approx_eq!(position.radius, 0.9830942391, 1e-9);
        let position = Planet::Earth.get_location(&JD::from(2159345.0));
        assert_approx_eq!(position.longitude.to_radians(), 1.8442244563, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), 0.0017331615, 1e-9);
        assert_approx_eq!(position.radius, 0.9830440401, 1e-9);
        let position = Planet::Earth.get_location(&JD::from(2122820.0));
        assert_approx_eq!(position.longitude.to_radians(), 1.8557201152, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), 0.0019445314, 1e-9);
        assert_approx_eq!(position.radius, 0.9830331809, 1e-9);
    }

    #[test]
    fn mars_position() {
        let position = Planet::Mars.get_location(&JD::from(2451545.0));
        assert_approx_eq!(position.longitude.to_radians(), 6.2735389872, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), -0.0247779824, 1e-9);
        assert_approx_eq!(position.radius, 1.3912076937, 1e-9);
        let position = Planet::Mars.get_location(&JD::from(2415020.0));
        assert_approx_eq!(position.longitude.to_radians(), 5.0185792656, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), -0.0274073500, 1e-9);
        assert_approx_eq!(position.radius, 1.4218777718, 1e-9);
        let position = Planet::Mars.get_location(&JD::from(2378495.0));
        assert_approx_eq!(position.longitude.to_radians(), 3.9199284825, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), 0.0031513365, 1e-9);
        assert_approx_eq!(position.radius, 1.5615140022, 1e-9);
        let position = Planet::Mars.get_location(&JD::from(2341970.0));
        assert_approx_eq!(position.longitude.to_radians(), 2.9897807830, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), 0.0280781217, 1e-9);
        assert_approx_eq!(position.radius, 1.6584697094, 1e-9);
        let position = Planet::Mars.get_location(&JD::from(2305445.0));
        assert_approx_eq!(position.longitude.to_radians(), 2.1032776583, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), 0.0308218939, 1e-9);
        assert_approx_eq!(position.radius, 1.6371997174, 1e-9);
        let position = Planet::Mars.get_location(&JD::from(2268920.0));
        assert_approx_eq!(position.longitude.to_radians(), 1.1268677424, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), 0.0077311848, 1e-9);
        assert_approx_eq!(position.radius, 1.5123622675, 1e-9);
        let position = Planet::Mars.get_location(&JD::from(2232395.0));
        assert_approx_eq!(position.longitude.to_radians(), 6.2441093266, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), -0.0266449540, 1e-9);
        assert_approx_eq!(position.radius, 1.3925964455, 1e-9);
        let position = Planet::Mars.get_location(&JD::from(2195870.0));
        assert_approx_eq!(position.longitude.to_radians(), 4.9898149167, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), -0.0270155266, 1e-9);
        assert_approx_eq!(position.radius, 1.4208707148, 1e-9);
        let position = Planet::Mars.get_location(&JD::from(2159345.0));
        assert_approx_eq!(position.longitude.to_radians(), 3.8886466318, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), 0.0052701267, 1e-9);
        assert_approx_eq!(position.radius, 1.5593802043, 1e-9);
        let position = Planet::Mars.get_location(&JD::from(2122820.0));
        assert_approx_eq!(position.longitude.to_radians(), 2.9557712523, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), 0.0298285524, 1e-9);
        assert_approx_eq!(position.radius, 1.6571002362, 1e-9);
    }

    #[test]
    fn jupiter_position() {
        let position = Planet::Jupiter.get_location(&JD::from(2451545.0));
        assert_approx_eq!(position.longitude.to_radians(), 0.6334614217, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), -0.0205001039, 1e-9);
        assert_approx_eq!(position.radius, 4.9653812803, 1e-9);
        let position = Planet::Jupiter.get_location(&JD::from(2415020.0));
        assert_approx_eq!(position.longitude.to_radians(), 4.1171308454, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), 0.0159456650, 1e-9);
        assert_approx_eq!(position.radius, 5.3850276351, 1e-9);
        let position = Planet::Jupiter.get_location(&JD::from(2378495.0));
        assert_approx_eq!(position.longitude.to_radians(), 1.5743114744, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), -0.0039059814, 1e-9);
        assert_approx_eq!(position.radius, 5.1318457347, 1e-9);
        let position = Planet::Jupiter.get_location(&JD::from(2341970.0));
        assert_approx_eq!(position.longitude.to_radians(), 4.9619913552, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), -0.0017586234, 1e-9);
        assert_approx_eq!(position.radius, 5.1888133706, 1e-9);
        let position = Planet::Jupiter.get_location(&JD::from(2305445.0));
        assert_approx_eq!(position.longitude.to_radians(), 2.4323346134, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), 0.0145957282, 1e-9);
        assert_approx_eq!(position.radius, 5.3439455250, 1e-9);
        let position = Planet::Jupiter.get_location(&JD::from(2268920.0));
        assert_approx_eq!(position.longitude.to_radians(), 5.8745612668, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), -0.0192161117, 1e-9);
        assert_approx_eq!(position.radius, 5.0018007431, 1e-9);
        let position = Planet::Jupiter.get_location(&JD::from(2232395.0));
        assert_approx_eq!(position.longitude.to_radians(), 3.2350793731, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), 0.0229002244, 1e-9);
        assert_approx_eq!(position.radius, 5.4491570418, 1e-9);
        let position = Planet::Jupiter.get_location(&JD::from(2195870.0));
        assert_approx_eq!(position.longitude.to_radians(), 0.5480874612, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), -0.0213293086, 1e-9);
        assert_approx_eq!(position.radius, 4.9715070844, 1e-9);
        let position = Planet::Jupiter.get_location(&JD::from(2159345.0));
        assert_approx_eq!(position.longitude.to_radians(), 4.0402354041, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), 0.0170598705, 1e-9);
        assert_approx_eq!(position.radius, 5.3896207448, 1e-9);
        let position = Planet::Jupiter.get_location(&JD::from(2122820.0));
        assert_approx_eq!(position.longitude.to_radians(), 1.4885071580, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), -0.0054711800, 1e-9);
        assert_approx_eq!(position.radius, 5.1193587263, 1e-9);
    }

    #[test]
    fn saturn_position() {
        let position = Planet::Saturn.get_location(&JD::from(2451545.0));
        assert_approx_eq!(position.longitude.to_radians(), 0.7980038867, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), -0.0401984149, 1e-9);
        assert_approx_eq!(position.radius, 9.1838482881, 1e-9);
        let position = Planet::Saturn.get_location(&JD::from(2415020.0));
        assert_approx_eq!(position.longitude.to_radians(), 4.6756597986, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), 0.0190423976, 1e-9);
        assert_approx_eq!(position.radius, 10.0668532372, 1e-9);
        let position = Planet::Saturn.get_location(&JD::from(2378495.0));
        assert_approx_eq!(position.longitude.to_radians(), 2.2444130058, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), 0.0107481008, 1e-9);
        assert_approx_eq!(position.radius, 9.1043067563, 1e-9);
        let position = Planet::Saturn.get_location(&JD::from(2341970.0));
        assert_approx_eq!(position.longitude.to_radians(), 5.8845121485, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), -0.0293639468, 1e-9);
        assert_approx_eq!(position.radius, 9.7629995093, 1e-9);
        let position = Planet::Saturn.get_location(&JD::from(2305445.0));
        assert_approx_eq!(position.longitude.to_radians(), 3.6192301828, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), 0.0432255907, 1e-9);
        assert_approx_eq!(position.radius, 9.7571035121, 1e-9);
        let position = Planet::Saturn.get_location(&JD::from(2268920.0));
        assert_approx_eq!(position.longitude.to_radians(), 0.9812189105, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), -0.0369435533, 1e-9);
        assert_approx_eq!(position.radius, 9.0669213668, 1e-9);
        let position = Planet::Saturn.get_location(&JD::from(2232395.0));
        assert_approx_eq!(position.longitude.to_radians(), 4.8374129245, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), 0.0133288784, 1e-9);
        assert_approx_eq!(position.radius, 10.1065693352, 1e-9);
        let position = Planet::Saturn.get_location(&JD::from(2195870.0));
        assert_approx_eq!(position.longitude.to_radians(), 2.4653200325, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), 0.0187797598, 1e-9);
        assert_approx_eq!(position.radius, 9.1857599387, 1e-9);
        let position = Planet::Saturn.get_location(&JD::from(2159345.0));
        assert_approx_eq!(position.longitude.to_radians(), 6.0607944160, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), -0.0336906976, 1e-9);
        assert_approx_eq!(position.radius, 9.5927174218, 1e-9);
        let position = Planet::Saturn.get_location(&JD::from(2122820.0));
        assert_approx_eq!(position.longitude.to_radians(), 3.7760794190, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), 0.0422300831, 1e-9);
        assert_approx_eq!(position.radius, 9.8669939127, 1e-9);
    }

    #[test]
    fn uranus_position() {
        let position = Planet::Uranus.get_location(&JD::from(2451545.0));
        assert_approx_eq!(position.longitude.to_radians(), 5.5225485297, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), -0.0119527878, 1e-9);
        assert_approx_eq!(position.radius, 19.9240478952, 1e-9);
        let position = Planet::Uranus.get_location(&JD::from(2415020.0));
        assert_approx_eq!(position.longitude.to_radians(), 4.3641525628, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), 0.0009368610, 1e-9);
        assert_approx_eq!(position.radius, 18.9927163179, 1e-9);
        let position = Planet::Uranus.get_location(&JD::from(2378495.0));
        assert_approx_eq!(position.longitude.to_radians(), 3.0875827851, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), 0.0132269524, 1e-9);
        assert_approx_eq!(position.radius, 18.2991151619, 1e-9);
        let position = Planet::Uranus.get_location(&JD::from(2341970.0));
        assert_approx_eq!(position.longitude.to_radians(), 1.7973185850, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), 0.0066373711, 1e-9);
        assert_approx_eq!(position.radius, 18.7966207283, 1e-9);
        let position = Planet::Uranus.get_location(&JD::from(2305445.0));
        assert_approx_eq!(position.longitude.to_radians(), 0.6197794565, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), -0.0084149193, 1e-9);
        assert_approx_eq!(position.radius, 19.7819885654, 1e-9);
        let position = Planet::Uranus.get_location(&JD::from(2268920.0));
        assert_approx_eq!(position.longitude.to_radians(), 5.8035494552, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), -0.0133827060, 1e-9);
        assert_approx_eq!(position.radius, 20.0300461334, 1e-9);
        let position = Planet::Uranus.get_location(&JD::from(2232395.0));
        assert_approx_eq!(position.longitude.to_radians(), 4.6715450662, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), -0.0033027749, 1e-9);
        assert_approx_eq!(position.radius, 19.2694312936, 1e-9);
        let position = Planet::Uranus.get_location(&JD::from(2195870.0));
        assert_approx_eq!(position.longitude.to_radians(), 3.4261485604, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), 0.0115506263, 1e-9);
        assert_approx_eq!(position.radius, 18.3948232424, 1e-9);
        let position = Planet::Uranus.get_location(&JD::from(2159345.0));
        assert_approx_eq!(position.longitude.to_radians(), 2.1281050050, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), 0.0103036775, 1e-9);
        assert_approx_eq!(position.radius, 18.5841497556, 1e-9);
        let position = Planet::Uranus.get_location(&JD::from(2122820.0));
        assert_approx_eq!(position.longitude.to_radians(), 0.9197293285, 1e-9);
        assert_approx_eq!(position.latitude.to_radians(), -0.0048575382, 1e-9);
        assert_approx_eq!(position.radius, 19.5612080219, 1e-9);
    }

    #[test]
    fn neptune_position() {
        let position = Planet::Neptune.get_location(&JD::from(2451545.0));
        assert_approx_eq!(position.longitude.to_radians(), 5.3045629284);
        assert_approx_eq!(position.latitude.to_radians(), 0.0042236790);
        assert_approx_eq!(position.radius, 30.1205329332);
        let position = Planet::Neptune.get_location(&JD::from(2415020.0));
        assert_approx_eq!(position.longitude.to_radians(), 1.5199957208);
        assert_approx_eq!(position.latitude.to_radians(), -0.0217331273);
        assert_approx_eq!(position.radius, 29.8710344515);
        let position = Planet::Neptune.get_location(&JD::from(2378495.0));
        assert_approx_eq!(position.longitude.to_radians(), 3.9778043127);
        assert_approx_eq!(position.latitude.to_radians(), 0.0307068993);
        assert_approx_eq!(position.radius, 30.3209191027);
        let position = Planet::Neptune.get_location(&JD::from(2341970.0));
        assert_approx_eq!(position.longitude.to_radians(), 0.1546340455);
        assert_approx_eq!(position.latitude.to_radians(), -0.0259181077);
        assert_approx_eq!(position.radius, 29.8685861463);
        let position = Planet::Neptune.get_location(&JD::from(2305445.0));
        assert_approx_eq!(position.longitude.to_radians(), 2.6511574700);
        assert_approx_eq!(position.latitude.to_radians(), 0.0106082425);
        assert_approx_eq!(position.radius, 30.1360158860);
        let position = Planet::Neptune.get_location(&JD::from(2268920.0));
        assert_approx_eq!(position.longitude.to_radians(), 5.0896381605);
        assert_approx_eq!(position.latitude.to_radians(), 0.0106592138);
        assert_approx_eq!(position.radius, 30.1785349992);
        let position = Planet::Neptune.get_location(&JD::from(2232395.0));
        assert_approx_eq!(position.longitude.to_radians(), 1.2984703832);
        assert_approx_eq!(position.latitude.to_radians(), -0.0260115821);
        assert_approx_eq!(position.radius, 29.8326055361);
        let position = Planet::Neptune.get_location(&JD::from(2195870.0));
        assert_approx_eq!(position.longitude.to_radians(), 3.7635416328);
        assert_approx_eq!(position.latitude.to_radians(), 0.0306777430);
        assert_approx_eq!(position.radius, 30.3109115122);
        let position = Planet::Neptune.get_location(&JD::from(2159345.0));
        assert_approx_eq!(position.longitude.to_radians(), 6.2151087391);
        assert_approx_eq!(position.latitude.to_radians(), -0.0215395777);
        assert_approx_eq!(position.radius, 29.9065506899);
        let position = Planet::Neptune.get_location(&JD::from(2122820.0));
        assert_approx_eq!(position.longitude.to_radians(), 2.4315044302);
        assert_approx_eq!(position.latitude.to_radians(), 0.0040125142);
        assert_approx_eq!(position.radius, 30.0653694889);
    }
}
