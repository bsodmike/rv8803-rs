use crate::Rv8803;
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

    /// Access the driver for the rtc clock.
    #[must_use]
    pub fn rtc(&self) -> Rv8803<crate::prelude::Bus<'_, shared_bus::I2cProxy<'_, SharedBusMutex>>> {
        let proxy = self.bus.acquire_i2c();

        Rv8803::from_i2c(proxy, self.device_address)
    }
}

/// The [`RTClock`] type is the interface for communicating with the `rv8803` rtc clock chip directly over I2C.
#[allow(dead_code)]
pub struct RTClockDirect<'a, I2C, I2cErr> {
    datetime: Option<DateTime<Utc>>,
    periph: I2C,
    bus_err: PhantomData<&'a I2cErr>,
    device_address: u8,
}

#[allow(dead_code)]
impl<'a, I2C, I2cErr> RTClockDirect<'a, I2C, I2cErr>
where
    I2C: embedded_hal_0_2::blocking::i2c::Write<Error = I2cErr>
        + embedded_hal_0_2::blocking::i2c::WriteRead<Error = I2cErr>,
    // I2cErr: defmt::Format,
{
    /// Creates a new [`RTClock`].
    pub fn new(periph: I2C, address: &u8) -> Self {
        Self {
            datetime: None,
            periph,
            bus_err: PhantomData,
            device_address: *address,
        }
    }

    /// Access the driver for the rtc clock.
    pub fn rtc(self) -> Rv8803<crate::prelude::Bus<'static, I2C>> {
        Rv8803::from_i2c(self.periph, self.device_address)
    }
}
