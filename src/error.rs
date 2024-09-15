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
pub enum DriverError<I2CError> {
    /// Error during I2C Transfer
    Transfer,

    #[allow(missing_docs)]
    _Phant(core::marker::PhantomData<I2CError>),
}

impl<I2CError> From<I2CError> for DriverError<I2CError> {
    fn from(_value: I2CError) -> Self {
        self::DriverError::Transfer
    }
}
