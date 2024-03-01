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
    /// Create an instance of [`Error`] specifying a [`Kind`] but without a cause.
    fn with_kind(kind: Kind) -> Self {
        Self {
            inner: Box::new(ErrorKind { kind, cause: None }),
        }
    }

    /// Create a new instance of [`Error`] with cause of type [`BoxError`]
    pub fn with<C: Into<BoxError>>(mut self, cause: C) -> Error {
        self.inner.cause = Some(cause.into());
        self
    }

    /// Create a new instance of [`Error`] of type [`Kind::InternalError`] with cause of type [`BoxError`]
    pub fn new<E: Into<BoxError>>(cause: E) -> Self {
        Self::default().with(cause)
    }
}

impl core::default::Default for Error {
    /// Create a new instance of [`Error`] of type [`Kind::InternalError`]
    fn default() -> Self {
        Self::with_kind(Kind::InternalError)
    }
}

#[cfg(feature = "linux_embedded_hal")]
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
