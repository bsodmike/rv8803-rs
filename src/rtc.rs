use crate::{driver::Driver, error::DriverError};
use chrono::{DateTime, Utc};
use core::marker::PhantomData;
#[allow(unused_imports)]
use defmt::error;
#[allow(unused_imports)]
use heapless::String;
use shared_bus::BusManager;

/// The [`RTClock`] type is the interface for communicating with the `rv8803` rtc clock chip via a shared bus over I2C.
#[allow(dead_code)]
pub struct RTClock<'a, I2C, I2cErr, M> {
    datetime: Option<DateTime<Utc>>,
    phantom: PhantomData<&'a I2C>,
    bus_err: PhantomData<&'a I2cErr>,
    bus: &'a BusManager<M>,
    device_address: u8,
    // driver_error: PhantomData<DriverError>,
}

#[allow(dead_code)]
impl<'a, I2C, I2cErr, SharedBusMutex> RTClock<'a, I2C, I2cErr, SharedBusMutex>
where
    I2C: embedded_hal_0_2::blocking::i2c::Write<Error = I2cErr>
        + embedded_hal_0_2::blocking::i2c::WriteRead<Error = I2cErr>,
    SharedBusMutex: shared_bus::BusMutex,
    <SharedBusMutex as shared_bus::BusMutex>::Bus: embedded_hal_0_2::blocking::i2c::Write<Error = I2cErr>
        + embedded_hal_0_2::blocking::i2c::WriteRead<Error = I2cErr>,
    I2cErr: defmt::Format,
    // + embedded_hal_0_2::blocking::i2c::WriteRead<Error = I2cErr>
    // + embedded_hal_0_2::blocking::i2c::Read<Error = I2cErr>
    // + embedded_hal_0_2::blocking::i2c::Write<Error = I2cErr>,
    DriverError<I2cErr>: From<I2cErr>,
{
    /// Creates a new [`RTClock`].
    pub fn new(bus: &'a BusManager<SharedBusMutex>, address: &u8) -> Self {
        Self {
            datetime: None,
            bus,
            phantom: PhantomData,
            bus_err: PhantomData,
            device_address: *address,
            // driver_error: PhantomData,
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
    ) -> Result<bool, DriverError<I2cErr>> {
        let proxy = self.bus.acquire_i2c();
        let mut driver = Driver::from_i2c(proxy, self.device_address);

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
    pub fn update_time(&mut self, dest: &mut [u8]) -> Result<bool, DriverError<I2cErr>> {
        let proxy = self.bus.acquire_i2c();
        let mut driver: Driver<crate::prelude::Bus<'_, shared_bus::I2cProxy<'_, SharedBusMutex>>> =
            Driver::from_i2c(proxy, self.device_address);

        Ok(driver.update_time(dest)?)
    }
}

/// The [`RTClock`] type is the interface for communicating with the `rv8803` rtc clock chip directly over I2C.
#[allow(dead_code)]
pub struct RTClockDirect<'a, I2C, I2cErr> {
    datetime: Option<DateTime<Utc>>,
    periph: I2C,
    bus_err: PhantomData<&'a I2cErr>,
    device_address: u8,
    // driver_error: PhantomData<DriverError>,
}

#[allow(dead_code)]
impl<'a, I2C, I2cErr> RTClockDirect<'a, I2C, I2cErr>
where
    I2C: embedded_hal_0_2::blocking::i2c::Write<Error = I2cErr>
        + embedded_hal_0_2::blocking::i2c::WriteRead<Error = I2cErr>,
    DriverError<I2cErr>: From<I2cErr>,
    // I2cErr: embedded_hal_0_2::blocking::i2c::WriteRead<Error = I2cErr>
    //     + embedded_hal_0_2::blocking::i2c::Read<Error = I2cErr>
    //     + embedded_hal_0_2::blocking::i2c::Write<Error = I2cErr>,
{
    /// Creates a new [`RTClock`].
    pub fn new(periph: I2C, address: &u8) -> Self {
        Self {
            datetime: None,
            periph,
            bus_err: PhantomData,
            device_address: *address,
            // driver_error: PhantomData,
        }
    }

    /// Access the driver for the rtc clock.
    pub fn rtc(self) -> Driver<crate::prelude::Bus<'static, I2C>> {
        Driver::from_i2c(self.periph, self.device_address)
    }
}
