/// All the models

/// Mapping of all the registers used to operate the RTC module
#[derive(Clone, Copy)]
pub enum Register {
    /// Hundreths
    Hundredths = 0x10,
    /// Seconds
    Seconds = 0x11,
    /// Minutes
    Minutes = 0x12,
    /// Hours
    Hours = 0x13,
    /// Weekday
    Weekday = 0x14,
    /// Date
    Date = 0x15,
    /// Month
    Month = 0x16,
    /// Year
    Year = 0x17,
    /// ControlReset
    ControlReset = 0,
    /// Control
    Control = 0x1F,
}

impl Register {
    /// Read address value, returns as [`u8`]
    pub fn address(&self) -> u8 {
        *self as u8
    }
}

/// Length of the time array constant
pub const TIME_ARRAY_LENGTH: usize = 8;

/// RV8803 driver.
#[derive(Debug)]
pub struct Rv8803<B> {
    /// Holds the bus.
    pub bus: B,
}
