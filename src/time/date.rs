//! Conversions to and from calendar dates
use super::JD;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Month {
    January = 1,
    February = 2,
    March = 3,
    April = 4,
    May = 5,
    June = 6,
    July = 7,
    August = 8,
    September = 9,
    October = 10,
    November = 11,
    December = 12,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DayOfWeek {
    Sunday = 0,
    Monday = 1,
    Tuesday = 2,
    Wednesday = 3,
    Thursday = 4,
    Friday = 5,
    Saturday = 6,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Calendar {
    Julian,
    Gregorian,
}

#[derive(Debug, Copy, Clone)]
pub struct Date {
    cal: Calendar,
    year: i32,
    month: Month,
    day: u8,
    fraction: f64,
}

impl Date {
    pub fn to_jd(&self) -> super::JD {
        let (y, m) = match self.month {
            Month::January | Month::February => (self.year as f64 - 1.0, self.month as i32 + 12),
            _ => (self.year as f64, self.month as i32),
        };

        let a = (y as f64 / 100.0).floor();
        let b = match self.cal {
            Calendar::Julian => 0.0,
            Calendar::Gregorian => 2.0 - a + (a / 4.0).floor(),
        };

        JD::from(
            (365.25f64 * (y + 4716.0)).floor()
                + (30.6001f64 * (m as f64 + 1.0)).floor()
                + (self.day as f64)
                + self.fraction
                + b
                - 1524.5,
        )
    }

    pub fn from_jd(jd: super::JD) -> Date {
        let z = (jd.to_f64() + 0.5).floor();
        let f = (jd.to_f64() + 0.5) - z;
        let a = if z < 2299_161.0 {
            z
        } else {
            let alpha = ((z - 1867_216.25) / 36524.25).floor();
            z + 1.0 + alpha - (alpha / 4.0).floor()
        };

        let b = a + 1524.0;
        let c = ((b - 122.1) / 365.25).floor();
        let d = (365.25 * c).floor();
        let e = ((b - d) / 30.6001).floor();

        let day_fraction = b - d - (30.6001 * e).floor() + f;
        let day = day_fraction.trunc();
        let fraction = day_fraction.fract();
        let month = if e < 14.0 {
            (e - 1.0) as i32
        } else {
            (e - 13.0) as i32
        };
        let year = if month > 2 {
            (c - 4716.0)
        } else {
            (c - 4715.0)
        };

        Date {
            cal: if jd.to_f64() >= 2299068.5 {
                Calendar::Gregorian
            } else {
                Calendar::Julian
            },
            year: year as i32,
            month: Month::from(month),
            day: day as u8,
            fraction,
        }
    }

    pub fn get_day_of_week(&self) -> DayOfWeek {
        let jd = self.to_jd().to_f64().round() - 0.5; // Rounded to the nearest day at 0h UTC
        let day_num = ((jd + 1.5) % 7.0).round() as i32;
        DayOfWeek::from(day_num)
    }

    pub fn get_day_of_year(&self) -> u16 {
        let leap_year = if let Calendar::Julian = self.cal {
            (self.year % 4) == 0
        } else {
            ((self.year % 4) == 0) && !((self.year % 400) == 0)
        };

        let k = if leap_year { 1 } else { 2 };

        let m = self.month as u16;
        let d = self.day as u16;

        (275 * m / 9) - k * ((m + 9) / 12) + d - 30
    }
}

impl From<i32> for Month {
    fn from(item: i32) -> Self {
        match item {
            1 => Month::January,
            2 => Month::February,
            3 => Month::March,
            4 => Month::April,
            5 => Month::May,
            6 => Month::June,
            7 => Month::July,
            8 => Month::August,
            9 => Month::September,
            10 => Month::October,
            11 => Month::November,
            12 => Month::December,
            _ => {
                assert!(false, "Invalid month number: {}", item);
                Month::January
            }
        }
    }
}

impl From<i32> for DayOfWeek {
    fn from(item: i32) -> Self {
        match item {
            0 => DayOfWeek::Sunday,
            1 => DayOfWeek::Monday,
            2 => DayOfWeek::Tuesday,
            3 => DayOfWeek::Wednesday,
            4 => DayOfWeek::Thursday,
            5 => DayOfWeek::Friday,
            6 => DayOfWeek::Saturday,
            _ => {
                assert!(false, "Invalid day number: {}", item);
                DayOfWeek::Sunday
            }
        }
    }
}

impl From<Date> for JD {
    fn from(item: Date) -> Self {
        item.to_jd()
    }
}

impl From<JD> for Date {
    fn from(item: JD) -> Self {
        Date::from_jd(item)
    }
}

#[cfg(test)]
mod tests {
    use super::{Calendar, Date, DayOfWeek, Month, JD};

    fn fraction_eq(frac1: f64, frac2: f64) -> bool {
        // 1 microsecond expressed in terms of fractions of a day, should be good enough for us
        let min_diff = 0.000001 * 60.0 * 60.0 * 24.0;
        (frac1 - frac2).abs() <= min_diff
    }

    #[test]
    fn date_to_jd() {
        // Example 7.a, page 61
        assert_eq!(
            Date {
                cal: Calendar::Gregorian,
                year: 1957,
                month: Month::October,
                day: 4,
                fraction: 0.81,
            }
            .to_jd(),
            2436116.31.into()
        );

        // Example 7.b, page 61
        assert_eq!(
            Date {
                cal: Calendar::Julian,
                year: 333,
                month: Month::January,
                day: 27,
                fraction: 0.5,
            }
            .to_jd(),
            1842713.0.into()
        );

        // Unlabeled table, page 62
        assert_eq!(
            Date {
                cal: Calendar::Gregorian,
                year: 2000,
                month: Month::January,
                day: 1,
                fraction: 0.5,
            }
            .to_jd(),
            2451545.0.into()
        );

        // Unlabeled table, page 62
        assert_eq!(
            Date {
                cal: Calendar::Gregorian,
                year: 1600,
                month: Month::December,
                day: 31,
                fraction: 0.0,
            }
            .to_jd(),
            2305812.5.into()
        );

        // Unlabeled table, page 62
        assert_eq!(
            Date {
                cal: Calendar::Julian,
                year: -1001,
                month: Month::August,
                day: 17,
                fraction: 0.9,
            }
            .to_jd(),
            1355671.4.into()
        );

        // Unlabeled table, page 62
        assert_eq!(
            Date {
                cal: Calendar::Julian,
                year: -4712,
                month: Month::January,
                day: 1,
                fraction: 0.5,
            }
            .to_jd(),
            0.0.into()
        );
    }

    #[test]
    fn jd_to_date() {
        // Example 7.c, page 64
        let d1 = Date::from_jd(JD::from(2436_116.31));
        assert_eq!(d1.cal, Calendar::Gregorian);
        assert_eq!(d1.year, 1957);
        assert_eq!(d1.month, Month::October);
        assert_eq!(d1.day, 4);
        assert!(fraction_eq(d1.fraction, 0.81));

        // Exercise, page 64
        let d1 = Date::from_jd(JD::from(1842_713.0));
        assert_eq!(d1.cal, Calendar::Julian);
        assert_eq!(d1.year, 333);
        assert_eq!(d1.month, Month::January);
        assert_eq!(d1.day, 27);
        assert!(fraction_eq(d1.fraction, 0.5));

        // Exercise, page 64
        let d1 = Date::from_jd(JD::from(1507_900.13));
        assert_eq!(d1.cal, Calendar::Julian);
        assert_eq!(d1.year, -584);
        assert_eq!(d1.month, Month::May);
        assert_eq!(d1.day, 28);
        assert!(fraction_eq(d1.fraction, 0.63));
    }

    #[test]
    fn day_of_week() {
        // Example 7.e, page 65
        assert_eq!(
            Date {
                cal: Calendar::Gregorian,
                year: 1954,
                month: Month::June,
                day: 30,
                fraction: 0.0
            }
            .get_day_of_week(),
            DayOfWeek::Wednesday
        );

        assert_eq!(
            Date {
                cal: Calendar::Gregorian,
                year: 2019,
                month: Month::July,
                day: 13,
                fraction: 0.0
            }
            .get_day_of_week(),
            DayOfWeek::Saturday
        );
        assert_eq!(
            Date {
                cal: Calendar::Gregorian,
                year: 2019,
                month: Month::July,
                day: 13,
                fraction: 0.4999999
            }
            .get_day_of_week(),
            DayOfWeek::Saturday
        );
        assert_eq!(
            Date {
                cal: Calendar::Gregorian,
                year: 2019,
                month: Month::July,
                day: 13,
                fraction: 0.5
            }
            .get_day_of_week(),
            DayOfWeek::Saturday
        );
        assert_eq!(
            Date {
                cal: Calendar::Gregorian,
                year: 2019,
                month: Month::July,
                day: 13,
                fraction: 0.9
            }
            .get_day_of_week(),
            DayOfWeek::Saturday
        );
        assert_eq!(
            Date {
                cal: Calendar::Gregorian,
                year: 2019,
                month: Month::July,
                day: 13,
                fraction: 0.99999999
            }
            .get_day_of_week(),
            DayOfWeek::Saturday
        );
    }

    #[test]
    fn day_of_year() {
        // Example 7.f, page 65
        assert_eq!(
            Date {
                cal: Calendar::Gregorian,
                year: 1978,
                month: Month::November,
                day: 14,
                fraction: 0.0
            }
            .get_day_of_year(),
            318
        );

        // Example 7.g, page 65
        assert_eq!(
            Date {
                cal: Calendar::Gregorian,
                year: 1988,
                month: Month::April,
                day: 22,
                fraction: 0.0
            }
            .get_day_of_year(),
            113
        );
    }
}
