#![feature(error_generic_member_access)]

use anyhow::Context;
use embedded_hal::i2c::{I2c, SevenBitAddress};
use error::Error;
use i2cdriver::I2CDriver;
use rv8803::experimental::{Driver, DriverTransferError};
use std::result::Result;

use embedded_hal_0_2::blocking::i2c::Read;

fn main() -> Result<(), Error> {
    let mut i2c: I2CDriver<i2cdriver::NormalMode> =
        I2CDriver::open("/dev/ttyUSB0").context("open I2CDriver")?;
    let device_address: u8 = 0x32;

    // let mut driver1: Driver<
    //     '_,
    //     I2CDriver<i2cdriver::NormalMode>,
    //     shared_bus::NullMutex<I2CDriver<i2cdriver::NormalMode>>,
    // > = Driver::using_periph(&mut i2c, &device_address);

    let mut driver1: Driver<'_, I2CDriver<i2cdriver::NormalMode>, _> =
        Driver::using_periph(&mut i2c, &device_address);

    // FIXME
    // let bm1: &mut shared_bus::BusManager<shared_bus::NullMutex<I2CDriver<i2cdriver::NormalMode>>> =
    //     driver1.bm();

    // let bm1: shared_bus::I2cProxy<'_, shared_bus::NullMutex<I2CDriver<i2cdriver::NormalMode>>> =
    //     driver1.bm().acquire_i2c();
    // let driver2 = Driver::using_periph(&mut i2c, &device_address);

    let mut buf = [0u8, 255];
    // let _ = bm1.read(device_address, &mut buf);

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
