use crate::driver::HandlesError;

/// Error type
#[derive(Debug)]
pub struct Error(dyn core::error::Error + Send + Sync);

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Error: {}", &self.0)
    }
}

impl core::error::Error for Error {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        self.0.source()
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn core::error::Error> {
        self.source()
    }

    fn provide<'a>(&'a self, _request: &mut core::error::Request<'a>) {}
}

/// Driver error.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[allow(clippy::module_name_repetitions)]
pub enum DriverError {
    /// When a checked integral type conversion fails.
    TryFromIntError(core::num::TryFromIntError),
}

impl HandlesError for DriverError {
    fn with(_err: &(dyn core::error::Error + Send + Sync)) {}
}

impl From<core::num::TryFromIntError> for DriverError {
    fn from(value: core::num::TryFromIntError) -> Self {
        self::DriverError::TryFromIntError(value)
    }
}
