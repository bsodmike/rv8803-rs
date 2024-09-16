#[cfg(feature = "alloc")]
use alloc::boxed::Box;

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

// NOTE: The error type in `embassy_stm32::i2c::Error` does not impl any traits (refer to https://docs.embassy.dev/embassy-stm32/git/stm32wl55cc-cm4/i2c/enum.Error.html), and since this is a driver, there is no context as to what specific driver is being used with this HAL this lib is depending on.
// Hence a generic type is used on `DriverTransferError<E>` to aid as a workaround.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[allow(clippy::module_name_repetitions)]
pub enum DriverTransferError<E> {
    /// Bus error occurred. e.g. A START or a STOP condition is detected and is not
    /// located after a multiple of 9 SCL clock pulses.
    Bus,
    /// The arbitration was lost, e.g. electrical problems with the clock signal.
    ArbitrationLoss,
    /// A bus operation was not acknowledged, e.g. due to the addressed device not
    /// being available on the bus or the device not being ready to process requests
    /// at the moment.
    NoAcknowledge(embedded_hal::i2c::NoAcknowledgeSource),
    /// The peripheral receive buffer was overrun.
    Overrun,
    /// A different error occurred. The original error may contain more information.
    Other,
    #[allow(missing_docs)]
    _Phant(core::marker::PhantomData<E>),
    /// Error during I2C Transfer
    Transfer,
    /// An unknown error has occurred.
    Unknown,
    /// Runtime error.
    #[cfg(feature = "alloc")]
    RunTime(Box<dyn embedded_hal::i2c::Error>),
    /// Runtime error when we want to erase the error value.  Bit of cheating here!
    RunTimeErased(E),
}

#[cfg(feature = "blocking")]
impl<E> From<E> for DriverTransferError<E> {
    fn from(_value: E) -> Self {
        self::DriverTransferError::Transfer
    }
}

#[cfg(feature = "async")]
impl<E> From<embedded_hal::i2c::ErrorKind> for DriverTransferError<E> {
    fn from(value: embedded_hal::i2c::ErrorKind) -> Self {
        match value {
            embedded_hal::i2c::ErrorKind::Bus => self::DriverTransferError::Bus,
            embedded_hal::i2c::ErrorKind::ArbitrationLoss => {
                self::DriverTransferError::ArbitrationLoss
            }
            embedded_hal::i2c::ErrorKind::NoAcknowledge(err) => {
                self::DriverTransferError::NoAcknowledge(err)
            }
            embedded_hal::i2c::ErrorKind::Overrun => self::DriverTransferError::Overrun,
            embedded_hal::i2c::ErrorKind::Other => self::DriverTransferError::Other,
            _ => self::DriverTransferError::Unknown,
        }
    }
}

#[cfg(feature = "async")]
impl<E> From<Box<dyn embedded_hal::i2c::Error>> for DriverTransferError<E> {
    fn from(value: Box<dyn embedded_hal::i2c::Error>) -> Self {
        self::DriverTransferError::RunTime(value)
    }
}

#[cfg(feature = "async")]
impl From<DriverTransferError<Box<dyn embedded_hal::i2c::Error>>> for DriverTransferError<()> {
    fn from(_value: DriverTransferError<Box<dyn embedded_hal::i2c::Error>>) -> Self {
        self::DriverTransferError::RunTimeErased(())
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[allow(clippy::module_name_repetitions)]
#[cfg(feature = "async")]
pub enum DriverAsyncError {
    Transfer(DriverTransferError<Box<dyn embedded_hal::i2c::Error>>),
}

#[cfg(feature = "async")]
impl From<DriverTransferError<Box<dyn embedded_hal::i2c::Error>>> for DriverAsyncError {
    fn from(value: DriverTransferError<Box<dyn embedded_hal::i2c::Error>>) -> Self {
        self::DriverAsyncError::Transfer(value)
    }
}
