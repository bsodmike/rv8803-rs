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

/// Driver transfer error.

// NOTE: This feels like a "shim" as the generic error type is always assumed to cause an I2C transfer error.  However, the type provided is just a struct from a dependent crate.  Sometimes these types do not have any traits, and since this is a driver, there is no context as to what specific driver is being used with this HAL this lib is depending on.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[allow(clippy::module_name_repetitions)]
pub enum DriverTransferError<E> {
    /// Error during I2C Transfer
    Transfer,

    #[allow(missing_docs)]
    _Phant(core::marker::PhantomData<E>),
}

impl<E> From<E> for DriverTransferError<E> {
    fn from(_value: E) -> Self {
        self::DriverTransferError::Transfer
    }
}
