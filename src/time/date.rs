//! Conversions to and from calendar dates
use crate::time::JD;

use core::fmt::{self, Display};

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum Calendar {
    Julian,
    Gregorian,
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Year(i32);

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
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

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct DayOfMonth(u8);

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum DayOfWeek {
    Sunday = 0,
    Monday = 1,
    Tuesday = 2,
    Wednesday = 3,
    Thursday = 4,
    Friday = 5,
    Saturday = 6,
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Date {
    cal: Calendar,
    year: Year,
    month: Month,
    day: DayOfMonth,
    fraction: f64,
}

impl Date {
    pub fn to_jd(&self) -> super::JD {
        let (y, m) = match self.month {
            Month::January | Month::February => (self.year.0 as f64 - 1.0, self.month as i32 + 12),
            _ => (self.year.0 as f64, self.month as i32),
        };

        let a = (y as f64 / 100.0).floor();
        let b = match self.cal {
            Calendar::Julian => 0.0,
            Calendar::Gregorian => 2.0 - a + (a / 4.0).floor(),
        };

        JD::from(
            (365.25f64 * (y + 4716.0)).floor()
                + (30.6001f64 * (m as f64 + 1.0)).floor()
                + (self.day.0 as f64)
                + self.fraction
                + b
                - 1524.5,
        )
    }

    pub fn from_jd(jd: super::JD) -> Date {
        let z = (jd.as_f64() + 0.5).floor();
        let f = (jd.as_f64() + 0.5) - z;
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
        let year = if month > 2 { c - 4716.0 } else { c - 4715.0 };

        Date {
            cal: if jd.as_f64() >= 2299068.5 {
                Calendar::Gregorian
            } else {
                Calendar::Julian
            },
            year: Year(year as i32),
            month: Month::from(month),
            day: DayOfMonth(day as u8),
            fraction,
        }
    }

    pub fn get_day_of_week(&self) -> DayOfWeek {
        let jd = self.to_jd().as_f64().round() - 0.5; // Rounded to the nearest day at 0h UTC
        let day_num = ((jd + 1.5) % 7.0).round() as i32;
        DayOfWeek::from(day_num)
    }

    pub fn get_day_of_year(&self) -> u16 {
        let leap_year = if let Calendar::Julian = self.cal {
            (self.year.0 % 4) == 0
        } else {
            ((self.year.0 % 4) == 0) && !((self.year.0 % 400) == 0)
        };

        let k = if leap_year { 1 } else { 2 };

        let m = self.month as u16;
        let d = self.day.0 as u16;

        (275 * m / 9) - k * ((m + 9) / 12) + d - 30
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} {}, {}", self.month, self.day.0, self.year.0)
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

impl From<i32> for Year {
    fn from(item: i32) -> Self {
        Self(item)
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

/// Calculates the date of Easter for a given year.
///
/// This function handles the differences in the Gregorian and Julian calendars, and uses 1583 as
/// the first year of the Gregorian calendar. It correctly calculates the date of easter for any
/// representable year.
///
/// #Note
/// The returned date has the fractional day set to `0.0`
pub fn find_easter_by_year(year: Year) -> Date {
    if year >= Year(1583) {
        find_gregorian_easter(year)
    } else {
        find_julian_easter(year)
    }
}

pub fn find_easter_by_calendar(year: Year, calendar: Calendar) -> Date {
    match calendar {
        Calendar::Julian => find_julian_easter(year),
        Calendar::Gregorian => find_gregorian_easter(year),
    }
}

/// Calculates the date of Easter for a given year in the Gregorian calendar
pub fn find_gregorian_easter(year: Year) -> Date {
    let a = year.0 % 19;
    let b = year.0 / 100;
    let c = year.0 % 100;
    let d = b / 4;
    let e = b % 4;
    let f = (b + 8) / 25;
    let g = (b - f + 1) / 3;
    let h = (19 * a + b - d - g + 15) % 30;
    let i = c / 4;
    let k = c % 4;
    let l = (32 + 2 * e + 2 * i - h - k) % 7;
    let m = (a + 11 * h + 22 * l) / 451;
    let n = (h + l - 7 * m + 114) / 31;
    let p = (h + l - 7 * m + 114) % 31;
    Date {
        cal: Calendar::Gregorian,
        year,
        month: n.into(),
        day: DayOfMonth((p + 1) as u8),
        fraction: 0.0,
    }
}

/// Calculates the date of Easter for a given year in the Julian calendar
pub fn find_julian_easter(year: Year) -> Date {
    let a = year.0 % 4;
    let b = year.0 % 7;
    let c = year.0 % 19;
    let d = (19 * c + 15) % 30;
    let e = (2 * a + 4 * b - d + 34) % 7;
    let f = (d + e + 114) / 31;
    let g = (d + e + 114) % 31;
    Date {
        cal: Calendar::Julian,
        year,
        month: f.into(),
        day: DayOfMonth((g + 1) as u8),
        fraction: 0.0,
    }
}

pub fn find_gregorian_passover(greg_year: Year) -> Date {
    let x = greg_year.0;
    let c = x / 100;
    let s = (3 * c - 5) / 4; // TODO: in Julian calendar use S = 0
    let _year = x + 3760; // Represents the year in the Jewish Calenday
    let a = (12 * x + 12) % 19;
    let b = x % 4;
    let q = -1.904_412_361_576 + 1.554_241_796_621 * a as f64 + 0.25 * b as f64
        - 0.003_177_794_022 * x as f64
        + s as f64;
    let q_int = q.trunc() as i32;
    let j = (q_int + 3 * x + 5 * b + 2 - s) % 7;
    let r = q - q.trunc();

    let d = if j == 2 || j == 4 || j == 6 {
        q_int + 23
    } else if j == 1 && a > 6 && r > 0.632_870_370 {
        q_int + 24
    } else if j == 0 && a > 11 && r >= 0.897_723_765 {
        q_int + 23
    } else {
        q_int + 22
    };

    let (month, day) = if d > 31 {
        (Month::April, DayOfMonth((d - 31) as u8))
    } else {
        (Month::March, DayOfMonth(d as u8))
    };

    Date {
        cal: Calendar::Gregorian,
        year: greg_year,
        month,
        day,
        fraction: 0.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
                year: Year(1957),
                month: Month::October,
                day: DayOfMonth(4),
                fraction: 0.81,
            }
            .to_jd(),
            2436116.31.into()
        );

        // Example 7.b, page 61
        assert_eq!(
            Date {
                cal: Calendar::Julian,
                year: Year(333),
                month: Month::January,
                day: DayOfMonth(27),
                fraction: 0.5,
            }
            .to_jd(),
            1842713.0.into()
        );

        // Unlabeled table, page 62
        assert_eq!(
            Date {
                cal: Calendar::Gregorian,
                year: Year(2000),
                month: Month::January,
                day: DayOfMonth(1),
                fraction: 0.5,
            }
            .to_jd(),
            2451545.0.into()
        );

        // Unlabeled table, page 62
        assert_eq!(
            Date {
                cal: Calendar::Gregorian,
                year: Year(1600),
                month: Month::December,
                day: DayOfMonth(31),
                fraction: 0.0,
            }
            .to_jd(),
            2305812.5.into()
        );

        // Unlabeled table, page 62
        assert_eq!(
            Date {
                cal: Calendar::Julian,
                year: Year(-1001),
                month: Month::August,
                day: DayOfMonth(17),
                fraction: 0.9,
            }
            .to_jd(),
            1355671.4.into()
        );

        // Unlabeled table, page 62
        assert_eq!(
            Date {
                cal: Calendar::Julian,
                year: Year(-4712),
                month: Month::January,
                day: DayOfMonth(1),
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
        assert_eq!(d1.year, Year(1957));
        assert_eq!(d1.month, Month::October);
        assert_eq!(d1.day, DayOfMonth(4));
        assert!(fraction_eq(d1.fraction, 0.81));

        // Exercise, page 64
        let d1 = Date::from_jd(JD::from(1842_713.0));
        assert_eq!(d1.cal, Calendar::Julian);
        assert_eq!(d1.year, Year(333));
        assert_eq!(d1.month, Month::January);
        assert_eq!(d1.day, DayOfMonth(27));
        assert!(fraction_eq(d1.fraction, 0.5));

        // Exercise, page 64
        let d1 = Date::from_jd(JD::from(1507_900.13));
        assert_eq!(d1.cal, Calendar::Julian);
        assert_eq!(d1.year, Year(-584));
        assert_eq!(d1.month, Month::May);
        assert_eq!(d1.day, DayOfMonth(28));
        assert!(fraction_eq(d1.fraction, 0.63));
    }

    #[test]
    fn day_of_week() {
        // Example 7.e, page 65
        assert_eq!(
            Date {
                cal: Calendar::Gregorian,
                year: Year(1954),
                month: Month::June,
                day: DayOfMonth(30),
                fraction: 0.0
            }
            .get_day_of_week(),
            DayOfWeek::Wednesday
        );

        assert_eq!(
            Date {
                cal: Calendar::Gregorian,
                year: Year(2019),
                month: Month::July,
                day: DayOfMonth(13),
                fraction: 0.0
            }
            .get_day_of_week(),
            DayOfWeek::Saturday
        );
        assert_eq!(
            Date {
                cal: Calendar::Gregorian,
                year: Year(2019),
                month: Month::July,
                day: DayOfMonth(13),
                fraction: 0.4999999
            }
            .get_day_of_week(),
            DayOfWeek::Saturday
        );
        assert_eq!(
            Date {
                cal: Calendar::Gregorian,
                year: Year(2019),
                month: Month::July,
                day: DayOfMonth(13),
                fraction: 0.5
            }
            .get_day_of_week(),
            DayOfWeek::Saturday
        );
        assert_eq!(
            Date {
                cal: Calendar::Gregorian,
                year: Year(2019),
                month: Month::July,
                day: DayOfMonth(13),
                fraction: 0.9
            }
            .get_day_of_week(),
            DayOfWeek::Saturday
        );
        assert_eq!(
            Date {
                cal: Calendar::Gregorian,
                year: Year(2019),
                month: Month::July,
                day: DayOfMonth(13),
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
                year: Year(1978),
                month: Month::November,
                day: DayOfMonth(14),
                fraction: 0.0
            }
            .get_day_of_year(),
            318
        );

        // Example 7.g, page 65
        assert_eq!(
            Date {
                cal: Calendar::Gregorian,
                year: Year(1988),
                month: Month::April,
                day: DayOfMonth(22),
                fraction: 0.0
            }
            .get_day_of_year(),
            113
        );
    }

    #[test]
    fn easter() {
        assert_eq!(
            find_easter_by_year(Year(1818)),
            Date {
                cal: Calendar::Gregorian,
                year: Year(1818),
                month: Month::March,
                day: DayOfMonth(22),
                fraction: 0.0
            }
        );
        assert_eq!(
            find_easter_by_year(Year(2285)),
            Date {
                cal: Calendar::Gregorian,
                year: Year(2285),
                month: Month::March,
                day: DayOfMonth(22),
                fraction: 0.0
            }
        );
        assert_eq!(
            find_easter_by_year(Year(1886)),
            Date {
                cal: Calendar::Gregorian,
                year: Year(1886),
                month: Month::April,
                day: DayOfMonth(25),
                fraction: 0.0
            }
        );
        assert_eq!(
            find_easter_by_year(Year(2038)),
            Date {
                cal: Calendar::Gregorian,
                year: Year(2038),
                month: Month::April,
                day: DayOfMonth(25),
                fraction: 0.0
            }
        );
        assert_eq!(
            find_easter_by_year(Year(1991)),
            Date {
                cal: Calendar::Gregorian,
                year: Year(1991),
                month: Month::March,
                day: DayOfMonth(31),
                fraction: 0.0
            }
        );
        assert_eq!(
            find_easter_by_year(Year(1992)),
            Date {
                cal: Calendar::Gregorian,
                year: Year(1992),
                month: Month::April,
                day: DayOfMonth(19),
                fraction: 0.0
            }
        );
        assert_eq!(
            find_easter_by_year(Year(1993)),
            Date {
                cal: Calendar::Gregorian,
                year: Year(1993),
                month: Month::April,
                day: DayOfMonth(11),
                fraction: 0.0
            }
        );
        assert_eq!(
            find_easter_by_year(Year(1954)),
            Date {
                cal: Calendar::Gregorian,
                year: Year(1954),
                month: Month::April,
                day: DayOfMonth(18),
                fraction: 0.0
            }
        );
        assert_eq!(
            find_easter_by_year(Year(2000)),
            Date {
                cal: Calendar::Gregorian,
                year: Year(2000),
                month: Month::April,
                day: DayOfMonth(23),
                fraction: 0.0
            }
        );
        assert_eq!(
            find_easter_by_year(Year(1818)),
            Date {
                cal: Calendar::Gregorian,
                year: Year(1818),
                month: Month::March,
                day: DayOfMonth(22),
                fraction: 0.0
            }
        );
        assert_eq!(
            find_easter_by_year(Year(179)),
            Date {
                cal: Calendar::Julian,
                year: Year(179),
                month: Month::April,
                day: DayOfMonth(12),
                fraction: 0.0
            }
        );
        assert_eq!(
            find_easter_by_year(Year(711)),
            Date {
                cal: Calendar::Julian,
                year: Year(711),
                month: Month::April,
                day: DayOfMonth(12),
                fraction: 0.0
            }
        );
        assert_eq!(
            find_easter_by_year(Year(1243)),
            Date {
                cal: Calendar::Julian,
                year: Year(1243),
                month: Month::April,
                day: DayOfMonth(12),
                fraction: 0.0
            }
        );
        assert_eq!(
            find_easter_by_year(Year(2023)),
            Date {
                cal: Calendar::Gregorian,
                year: Year(2023),
                month: Month::April,
                day: DayOfMonth(9),
                fraction: 0.0
            }
        );
        assert_eq!(
            find_easter_by_year(Year(2024)),
            Date {
                cal: Calendar::Gregorian,
                year: Year(2024),
                month: Month::March,
                day: DayOfMonth(31),
                fraction: 0.0
            }
        );
        assert_eq!(
            find_easter_by_year(Year(2025)),
            Date {
                cal: Calendar::Gregorian,
                year: Year(2025),
                month: Month::April,
                day: DayOfMonth(20),
                fraction: 0.0
            }
        );
    }

    #[test]
    fn passover() {
        assert_eq!(
            find_gregorian_passover(Year(1990)),
            Date {
                cal: Calendar::Gregorian,
                year: Year(1990),
                month: Month::April,
                day: DayOfMonth(10),
                fraction: 0.0
            }
        );

        assert_eq!(
            find_gregorian_passover(Year(2023)),
            Date {
                cal: Calendar::Gregorian,
                year: Year(2023),
                month: Month::April,
                day: DayOfMonth(6),
                fraction: 0.0
            }
        );

        assert_eq!(
            find_gregorian_passover(Year(2024)),
            Date {
                cal: Calendar::Gregorian,
                year: Year(2024),
                month: Month::April,
                day: DayOfMonth(23),
                fraction: 0.0
            }
        );

        assert_eq!(
            find_gregorian_passover(Year(2025)),
            Date {
                cal: Calendar::Gregorian,
                year: Year(2025),
                month: Month::April,
                day: DayOfMonth(13),
                fraction: 0.0
            }
        );
    }
}
