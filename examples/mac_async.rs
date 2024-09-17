#![feature(error_generic_member_access)]

use anyhow::Context;
use embedded_hal::i2c::{I2c, SevenBitAddress};
use error::Error;
use i2cdriver::I2CDriver;
use rv8803::experimental::{Driver, DriverTransferError};
use std::result::Result;

fn main() -> Result<(), Error> {
    let mut i2c = I2CDriver::open("/dev/ttyUSB0").context("open I2CDriver")?;
    let device_address: u8 = 0x32;

    let driver1 = Driver::using_periph(&mut i2c, &device_address);
    // let driver2 = Driver::using_periph(&mut i2c, &device_address);

    let mut buf = [0u8, 255];
    let _ = driver1.periph.read(device_address, &mut buf);

    Ok(())
}

mod error {
    /// Boxed error type
    type BoxError = Box<dyn core::error::Error + Send + Sync>;

    #[derive(Debug)]
    pub(crate) struct Error(BoxError);

    impl core::fmt::Display for Error {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "Error: {}", self.0)
        }
    }

    impl From<anyhow::Error> for Error {
        fn from(value: anyhow::Error) -> Self {
            Error(value.into())
        }
    }

    impl core::error::Error for Error {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            self.0.source()
        }

        fn description(&self) -> &str {
            "description() is deprecated; use Display"
        }

        fn cause(&self) -> Option<&dyn std::error::Error> {
            self.source()
        }

        fn provide<'a>(&'a self, _request: &mut std::error::Request<'a>) {}
    }
}
