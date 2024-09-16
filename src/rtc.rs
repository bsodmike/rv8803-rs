#[allow(unused_imports)]
use crate::{driver::Driver, error::DriverTransferError};
use chrono::{DateTime, Utc};
use core::marker::PhantomData;
#[cfg(feature = "defmt")]
#[allow(unused_imports)]
use defmt::error;
#[allow(unused_imports)]
use heapless::String;
use shared_bus::BusManager;

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

/// The [`RTClock`] type is the interface for communicating with the `rv8803` rtc clock chip via a shared bus over I2C.
#[allow(dead_code)]
pub struct RTClock<'a, I2C, I2cErr, M> {
    datetime: Option<DateTime<Utc>>,
    phantom: PhantomData<&'a I2C>,
    bus_err: PhantomData<&'a I2cErr>,
    bus: &'a BusManager<M>,
    device_address: u8,
}

#[cfg(all(feature = "blocking", feature = "defmt"))]
#[allow(dead_code)]
impl<'a, I2C, I2cErr, SharedBusMutex> RTClock<'a, I2C, I2cErr, SharedBusMutex>
where
    I2C: embedded_hal_0_2::blocking::i2c::Write<Error = I2cErr>
        + embedded_hal_0_2::blocking::i2c::WriteRead<Error = I2cErr>,
    SharedBusMutex: shared_bus::BusMutex,
    <SharedBusMutex as shared_bus::BusMutex>::Bus: embedded_hal_0_2::blocking::i2c::Write<Error = I2cErr>
        + embedded_hal_0_2::blocking::i2c::WriteRead<Error = I2cErr>,
    I2cErr: defmt::Format,
    DriverTransferError<I2cErr>: From<I2cErr>,
{
    /// Creates a new [`RTClock`].
    pub fn new(bus: &'a BusManager<SharedBusMutex>, address: &u8) -> Self {
        Self {
            datetime: None,
            bus,
            phantom: PhantomData,
            bus_err: PhantomData,
            device_address: *address,
        }
    }

    /// Set time on the Driver module
    ///
    /// # Errors
    ///
    /// Read/write errors during communication with the `rv8803` chip will return an error.
    #[allow(clippy::too_many_arguments)]
    pub fn set_time(
        &mut self,
        sec: u8,
        min: u8,
        hour: u8,
        weekday: u8,
        date: u8,
        month: u8,
        year: u16,
    ) -> Result<bool, DriverTransferError<I2cErr>> {
        let proxy = self.bus.acquire_i2c();
        let mut driver = Driver::new(proxy, self.device_address);

        match driver.set_time(sec, min, hour, weekday, date, month, year) {
            Ok(val) => Ok(val),
            Err(err) => Err(err),
        }
    }

    /// Fetch time from the RTC clock and store it in the buffer `dest`.
    ///
    /// # Errors
    ///
    /// Read/write errors during communication with the `rv8803` chip will return an error.
    pub fn update_time(&mut self, dest: &mut [u8]) -> Result<bool, DriverTransferError<I2cErr>> {
        let proxy = self.bus.acquire_i2c();
        let mut driver: Driver<crate::prelude::Bus<'_, shared_bus::I2cProxy<'_, SharedBusMutex>>> =
            Driver::new(proxy, self.device_address);

        Ok(driver.update_time(dest)?)
    }
}

/// The [`RTClockDirect`] type is the interface for communicating with the `rv8803` rtc clock chip directly over I2C.
#[cfg(feature = "blocking")]
#[allow(dead_code)]
pub struct RTClockDirect<'a, I2C, I2cErr> {
    datetime: Option<DateTime<Utc>>,
    periph: I2C,
    bus_err: PhantomData<&'a I2cErr>,
    device_address: u8,
}

#[cfg(feature = "blocking")]
#[allow(dead_code)]
impl<'a, I2C, I2cErr> RTClockDirect<'a, I2C, I2cErr>
where
    I2C: embedded_hal_0_2::blocking::i2c::Write<Error = I2cErr>
        + embedded_hal_0_2::blocking::i2c::WriteRead<Error = I2cErr>,
    DriverTransferError<I2cErr>: From<I2cErr>,
{
    /// Creates a new [`RTClockDirect`].
    pub fn new(periph: I2C, address: &u8) -> Self {
        Self {
            datetime: None,
            periph,
            bus_err: PhantomData,
            device_address: *address,
        }
    }

    /// Fetch time from the RTC clock and store it in the buffer `dest`.
    ///
    /// # Errors
    ///
    /// Read/write errors during communication with the `rv8803` chip will return an error.
    pub fn update_time(self, dest: &mut [u8]) -> Result<bool, DriverTransferError<I2cErr>> {
        let mut driver = Driver::new(self.periph, self.device_address);

        Ok(driver.update_time(dest)?)
    }
}

/// The [`RTClockDirect`] type is the interface for communicating with the `rv8803` rtc clock chip directly over I2C.
#[cfg(feature = "async")]
#[allow(dead_code)]
pub struct RTClockDirect<I2C> {
    datetime: Option<DateTime<Utc>>,
    periph: I2C,
    device_address: u8,
}

#[cfg(feature = "async")]
#[allow(dead_code)]
impl<I2C> RTClockDirect<I2C>
where
    I2C: embedded_hal::i2c::I2c<Error = Box<dyn embedded_hal::i2c::Error>>,
{
    /// Creates a new [`RTClockDirect`].
    pub fn new(periph: I2C, address: &u8) -> Self {
        Self {
            datetime: None,
            periph,
            device_address: *address,
        }
    }

    /// Set time on the Driver module
    ///
    /// # Errors
    ///
    /// Read/write errors during communication with the `rv8803` chip will return an error.
    #[allow(clippy::too_many_arguments)]
    pub fn set_time(
        &mut self,
        sec: u8,
        min: u8,
        hour: u8,
        weekday: u8,
        date: u8,
        month: u8,
        year: u16,
    ) -> Result<bool, DriverTransferError<()>> {
        let periph = &mut self.periph;
        let mut driver = Driver::from_i2c(periph, self.device_address);

        match driver.set_time(sec, min, hour, weekday, date, month, year) {
            Ok(val) => Ok(val),
            Err(err) => Err(err),
        }
    }

    /// Fetch time from the RTC clock and store it in the buffer `dest`.
    ///
    /// # Errors
    ///
    /// Read/write errors during communication with the `rv8803` chip will return an error.
    #[allow(clippy::needless_question_mark)]
    pub fn update_time(&mut self, dest: &mut [u8]) -> Result<bool, DriverTransferError<()>> {
        let periph = &mut self.periph;
        let mut driver = Driver::from_i2c(periph, self.device_address);

        Ok(driver.update_time(dest)?)
    }
}
