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
}

/// Driver error.
#[derive(Debug)]
#[allow(clippy::module_name_repetitions)]
pub enum DriverError<E> {
    /// I2C bus error
    I2c(E),
}

impl<E> From<E> for DriverError<E> {
    fn from(other: E) -> Self {
        self::DriverError::I2c(other)
    }
}
