use crate::{formatter::ByteMutWriter, log::LoggableClockData};
use core::fmt::{Debug, Write};

/// Holds the clock data.
#[derive(Debug, Copy, Clone, Default)]
pub struct ClockData {
    /// Hundredths.
    pub hundredths: u8,
    /// Seconds.
    pub seconds: u8,
    /// Minutes.
    pub minutes: u8,
    /// Hours.
    pub hours: u8,
    /// Weekday.
    pub weekday: u8,
    /// Date.
    pub date: u8,
    /// Month.
    pub month: u8,
    /// Year.
    pub year: u8,
}

impl ClockData {
    /// Creates a [`ClockData`].
    #[must_use]
    pub fn new() -> ClockData {
        ClockData {
            ..Default::default()
        }
    }

    /// Hundredths.
    #[must_use]
    pub fn hundredths(&self) -> u8 {
        self.hundredths
    }

    /// Seconds.
    #[must_use]
    pub fn seconds(&self) -> u8 {
        self.seconds
    }

    /// Minutes.
    #[must_use]
    pub fn minutes(&self) -> u8 {
        self.minutes
    }

    /// Hours.
    #[must_use]
    pub fn hours(&self) -> u8 {
        self.hours
    }

    /// Weekday.
    #[must_use]
    pub fn weekday(&self) -> u8 {
        self.weekday
    }

    /// Day. The rtc module refers to this as "date".
    #[must_use]
    pub fn day(&self) -> u8 {
        self.date
    }

    /// Month.
    #[must_use]
    pub fn month(&self) -> u8 {
        self.month
    }

    /// Year.
    #[must_use]
    pub fn year(&self) -> u8 {
        self.year
    }

    /// Set the date and time.  Hundredths is set to 0.
    pub fn set(&mut self, value: (u8, u8, u8, Weekday, u8, Month, CurrentYear)) {
        let (hours, minutes, seconds, weekday, day, month, year) = value;

        self.hundredths = 0;
        self.hours = hours;
        self.minutes = minutes;
        self.seconds = seconds;
        self.weekday = weekday as u8;
        self.date = day;
        self.month = month as u8;
        self.year = year.0;
    }
}

/// Creates a tuple to hold the current year.
pub struct CurrentYear(u8);

impl CurrentYear {
    /// Provides the current year as a [`u8`].
    pub fn new(value: u16) -> Self {
        if value < 1970 {
            panic!("Year must be greater than 1970");
        }

        if value < 2000 {
            Self((value - 1900) as u8)
        } else {
            Self((value - 2000) as u8)
        }
    }
}

impl defmt::Format for LoggableClockData {
    fn format(&self, fmt: defmt::Formatter) {
        let data = self.data();

        let mut buf = [0u8; 2];
        let mut buf = ByteMutWriter::new(&mut buf[..]);
        let hours = left_pad(&mut buf, data.hours);

        let mut buf = [0u8; 2];
        let mut buf = ByteMutWriter::new(&mut buf[..]);
        let minutes = left_pad(&mut buf, data.minutes);

        let mut buf = [0u8; 2];
        let mut buf = ByteMutWriter::new(&mut buf[..]);
        let seconds = left_pad(&mut buf, data.seconds);

        let mut buf = [0u8; 2];
        let mut buf = ByteMutWriter::new(&mut buf[..]);
        let day = left_pad(&mut buf, data.date);

        let month = Month::from(data.month);
        let weekday = Weekday::from(data.weekday);

        let mut buf = [0u8; 4];
        let mut buf = ByteMutWriter::new(&mut buf[..]);
        let year = pad_year(&mut buf, data.year, self.century());

        defmt::write!(
            fmt,
            "{}:{}:{}, {}, {} {} {}",
            hours,
            minutes,
            seconds,
            weekday,
            day,
            month,
            year,
        );
    }
}

/// Enumerated type values for the weekday register.
#[allow(dead_code)]
pub enum Weekday {
    /// Sunday
    Sunday = 1,
    /// Monday
    Monday = 2,
    /// Tuesday
    Tuesday = 4,
    /// Wednesday
    Wednesday = 8,
    /// Thursday
    Thursday = 16,
    /// Friday
    Friday = 32,
    /// Saturday
    Saturday = 64,
}

// FIXME this causes [defmt] to get stuck. Why?
// impl From<u8> for Weekday {
//     fn from(value: u8) -> Self {
//         Self::into(value.into())
//     }
// }
#[allow(dead_code)]
#[allow(clippy::match_same_arms)]
impl Weekday {
    /// Get variant from a provided value.
    #[must_use]
    pub fn from(val: u8) -> Self {
        match val {
            1 => Self::Sunday,
            2 => Self::Monday,
            4 => Self::Tuesday,
            8 => Self::Wednesday,
            16 => Self::Thursday,
            32 => Self::Friday,
            64 => Self::Saturday,
            _ => Self::Sunday,
        }
    }
}

impl defmt::Format for Weekday {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "{}",
            match self {
                Self::Sunday => "Sunday",
                Self::Monday => "Monday",
                Self::Tuesday => "Tuesday",
                Self::Wednesday => "Wednesday",
                Self::Thursday => "Thursday",
                Self::Friday => "Friday",
                Self::Saturday => "Saturday",
            }
        );
    }
}

/// Enumerated type values for the month register.
#[allow(dead_code)]
pub enum Month {
    /// January
    January = 1,
    /// February
    February = 2,
    /// March
    March = 3,
    /// April
    April = 4,
    /// May
    May = 5,
    /// June
    June = 6,
    /// July
    July = 7,
    /// August
    August = 8,
    /// September
    September = 9,
    /// October
    October = 10,
    /// November
    November = 11,
    /// December
    December = 12,
}

#[allow(dead_code)]
#[allow(clippy::match_same_arms)]
impl Month {
    /// Get variant from a provided value.
    #[must_use]
    pub fn from(val: u8) -> Self {
        match val {
            1 => Self::January,
            2 => Self::February,
            3 => Self::March,
            4 => Self::April,
            5 => Self::May,
            6 => Self::June,
            7 => Self::July,
            8 => Self::August,
            9 => Self::September,
            10 => Self::October,
            11 => Self::November,
            12 => Self::December,
            _ => Self::January,
        }
    }
}

impl defmt::Format for Month {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "{}",
            match self {
                Self::January => "January",
                Self::February => "February",
                Self::March => "March",
                Self::April => "April",
                Self::May => "May",
                Self::June => "June",
                Self::July => "July",
                Self::August => "August",
                Self::September => "September",
                Self::October => "October",
                Self::November => "November",
                Self::December => "December",
            }
        );
    }
}

/// Enumerated type values for the year.
#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
pub enum Year {
    /// 20th Century Fox. Definitely, better times.
    TwentiethCentury(u8),
    /// Reality.
    TwentyFirstCentury(u8),
}

impl Default for Year {
    fn default() -> Self {
        Self::TwentyFirstCentury(20)
    }
}

fn left_pad<'a>(buf: &'a mut ByteMutWriter<'_>, value: u8) -> &'a str {
    buf.clear();
    write!(buf, "{}{}", common_padding(value), value).unwrap();

    buf.as_str()
}

fn pad_year<'a>(buf: &'a mut ByteMutWriter<'_>, value: u8, century: Year) -> &'a str {
    buf.clear();

    match century {
        Year::TwentiethCentury(_) => write!(buf, "19{}{}", common_padding(value), value).unwrap(),
        Year::TwentyFirstCentury(_) => write!(buf, "20{}{}", common_padding(value), value).unwrap(),
    }

    buf.as_str()
}

fn common_padding<'a>(value: u8) -> &'a str {
    if value < 10 {
        "0"
    } else {
        ""
    }
}

#[allow(dead_code)]
pub mod misc {
    pub fn bcd_to_dec(value: u8) -> u8 {
        ((value / 0x10) * 10) + (value % 0x10)
    }

    pub fn dec_to_bcd(value: u8) -> u8 {
        ((value / 10) * 0x10) + (value % 10)
    }
}
