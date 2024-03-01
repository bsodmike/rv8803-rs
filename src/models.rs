/// All possible errors in this crate
use alloc::boxed::Box;
use core::{
    error::{self},
    fmt::Display,
};

/// Type for all crate errors
pub type CrateError = Error;
/// Boxed error type
pub type BoxError = Box<dyn error::Error + Send + Sync>;

#[allow(dead_code)]
#[derive(Debug)]
/// Error struct
pub struct Error {
    inner: Box<ErrorKind>,
}

#[allow(dead_code)]
#[derive(Debug)]
/// Error kind record struct holding both [`Kind`] and an [`Option`]< [`BoxError`]>
struct ErrorKind {
    kind: Kind,
    cause: Option<BoxError>,
}

/// Error kind enum
#[derive(Debug)]
pub enum Kind {
    /// Default crate error.
    InternalError,
}

impl Error {
    /// Create a new instance of [`Error`] without a cause.
    fn new(kind: Kind) -> Self {
        Self {
            inner: Box::new(ErrorKind { kind, cause: None }),
        }
    }

    /// Create a new instance of [`Error`] with cause of type [`BoxError`]
    pub fn with<C: Into<BoxError>>(mut self, cause: C) -> Error {
        self.inner.cause = Some(cause.into());
        self
    }

    /// Create a new instance of [`Error`] of type [`Kind::InternalError`]
    pub fn default_err() -> Self {
        Self::new(Kind::InternalError)
    }

    /// Create a new instance of [`Error`] of type [`Kind::InternalError`] with cause of type [`BoxError`]
    pub fn default_err_with_cause<E: Into<BoxError>>(cause: E) -> Self {
        Self::default_err().with(cause)
    }
}

#[cfg(feature = "linux_embedded_hal")]
impl FnOnce<(linux_embedded_hal::i2cdev::linux::LinuxI2CError,)> for Error {
    type Output = Error;

    extern "rust-call" fn call_once(
        self,
        args: (linux_embedded_hal::i2cdev::linux::LinuxI2CError,),
    ) -> Self::Output {
        Error::default_err_with_cause(args.0)
    }
}

impl Display for CrateError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if let Some(ref cause) = self.inner.cause {
            write!(f, "CrateError: {}", cause)
        } else {
            f.write_str("CrateError: Unknown error")
        }
    }
}

impl error::Error for CrateError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        self.inner
            .cause
            .as_ref()
            .map(|cause| &**cause as &(dyn error::Error + 'static))
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
