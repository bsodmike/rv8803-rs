/// All possible errors in this crate
use alloc::boxed::Box;
use core::{error, fmt::Display};

/// Type for all crate errors
pub type CrateError = Error;
/// Boxed error type
pub type BoxError = Box<dyn error::Error + Send + Sync>;

#[allow(dead_code)]
#[derive(Debug)]
/// Error struct
pub struct Error {
    inner: BoxError,
}

impl Error {
    /// Create a new instance of [`Error`]
    pub fn new(error: impl Into<BoxError>) -> Self {
        Self {
            inner: error.into(),
        }
    }

    /// Create a default instance of [`Error`]
    #[allow(clippy::should_implement_trait)]
    #[allow(clippy::unnecessary_literal_unwrap)]
    pub fn default() -> Self {
        Self {
            // FIXME: isn't this the same as `panic!("{:?}", ())`??
            inner: Err(()).unwrap(),
        }
    }
}

impl FnOnce<(linux_embedded_hal::i2cdev::linux::LinuxI2CError,)> for Error {
    type Output = Error;

    extern "rust-call" fn call_once(
        self,
        args: (linux_embedded_hal::i2cdev::linux::LinuxI2CError,),
    ) -> Self::Output {
        Error::new(args.0)
    }
}

impl Display for CrateError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.inner.fmt(f)
    }
}

impl error::Error for CrateError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(&*self.inner)
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        self.source()
    }

    fn provide<'a>(&'a self, _request: &mut core::error::Request<'a>) {}
}

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
