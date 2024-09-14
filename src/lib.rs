//! RV8803 driver for I2C.
#![no_std]
#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(feature = "async", allow(incomplete_features))]
#![cfg_attr(docsrs, feature(doc_cfg), feature(doc_auto_cfg))]
#![feature(error_generic_member_access)]
#![feature(trivial_bounds)]

use crate::bus::{Bus, BusTrait};
pub use embedded_hal_0_2;
use models::{Register, Rv8803, TIME_ARRAY_LENGTH};

/// RV8803 I2C bus implementation with embedded-hal version 0.2
pub mod bus;

/// Models
pub(crate) mod models;

pub(crate) mod error;

/// Module for [`crate::rtc::RTClock`]
pub mod rtc;

/// Re-exports
pub mod prelude {
    pub use crate::bus::Bus;
    pub use crate::error::Error;
}

#[allow(dead_code)]
impl<'a, I2C, E> Rv8803<Bus<'a, I2C>>
where
    I2C: embedded_hal_0_2::blocking::i2c::WriteRead<Error = E>
        + embedded_hal_0_2::blocking::i2c::Write<Error = E>,
    Bus<'a, I2C>: bus::BusTrait<Error = E>,
{
    /// Create a new RV8803 from a [`crate::driver::Bus`].
    ///
    /// # Errors
    ///
    /// If this function encounters an error, it will be returned.
    pub fn new(bus: Bus<'a, I2C>) -> Self {
        Self { bus }
    }

    /// Creates a new `Rv8803` driver from a I2C peripheral, and an I2C
    /// device address.
    pub fn from_i2c(i2c: I2C, address: u8) -> Self {
        let bus = crate::bus::Bus::new(i2c, &address);

        Self::new(bus)
    }

    /// Creates a new `Rv8803` driver from a I2C peripheral, and an I2C
    /// device address.
    pub fn from_bus(i2c: I2C, address: u8) -> Self {
        let bus = crate::bus::Bus::new(i2c, &address);

        Self::new(bus)
    }

    /// Set time on the RV8803 module
    ///
    /// # Errors
    ///
    /// If the year specified is always > 2000, hence `u16` casting to `u8`
    /// is OK, as long as the year is < 2100. When the year > 2255 this will
    /// return [`core::num::TryFromIntError`].
    ///
    /// Read/write errors during communication with the `rv8803` chip will also return an error.
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
    ) -> Result<bool, E>
    where
        E: core::convert::From<core::num::TryFromIntError>,
    {
        self.bus
            .write_register(Register::Seconds, dec_to_bcd(sec))?;
        self.bus
            .write_register(Register::Minutes, dec_to_bcd(min))?;
        self.bus.write_register(Register::Hours, dec_to_bcd(hour))?;
        self.bus.write_register(Register::Date, dec_to_bcd(date))?;
        self.bus
            .write_register(Register::Month, dec_to_bcd(month))?;
        self.bus
            .write_register(Register::Year, dec_to_bcd(u8::try_from(year - 2000)?))?;
        self.bus.write_register(Register::Weekday, weekday)?;

        // Set RESET bit to 0 after setting time to make sure seconds don't get stuck.
        self.write_bit(
            Register::Control.address(),
            Register::ControlReset.address(),
            false,
        )?;

        defmt::debug!("rv8803::set_time: updated RTC clock");

        Ok(true)
    }

    /// Fetch time from the RTC clock and store it in the buffer `dest`.
    pub fn update_time(&mut self, dest: &mut [u8]) -> Result<bool, E> {
        if !(self.bus.read_multiple_registers(
            Register::Hundredths.address(),
            dest,
            TIME_ARRAY_LENGTH,
        )?) {
            defmt::warn!("update_time: attempt read - fail 1");
            return Ok(false); // Something went wrong
        }

        // If hundredths are at 99 or seconds are at 59, read again to make sure we didn't accidentally skip a second/minute
        if bcd_to_dec(dest[0]) == 99 || bcd_to_dec(dest[1]) == 59 {
            let mut temp_time = [0_u8; TIME_ARRAY_LENGTH];

            defmt::debug!("update_time: if hundredths are at 99 or seconds are at 59, read again to make sure we didn't accidentally skip a second/minute / Hundreths: {} / Seconds: {}", bcd_to_dec(dest[0]),bcd_to_dec(dest[1]));

            if !(self.bus.read_multiple_registers(
                Register::Hundredths.address(),
                &mut temp_time,
                TIME_ARRAY_LENGTH,
            )?) {
                defmt::warn!("update_time: attempt read - fail 2");
                return Ok(false); // Something went wrong
            };

            // If the reading for hundredths has rolled over, then our new data is correct, otherwise, we can leave the old data.
            if bcd_to_dec(dest[0]) > bcd_to_dec(temp_time[0]) {
                defmt::debug!("update_time: the reading for hundredths has rolled over, then our new data is correct. / Hundreths: {} / temp_time[0]: {}",
                bcd_to_dec(dest[0]),
                bcd_to_dec(temp_time[0]));

                for (i, el) in temp_time.iter().enumerate() {
                    dest[i] = *el;
                }
            }
        }

        // byte order: https://github.com/sparkfun/SparkFun_RV-8803_Arduino_Library/blob/main/src/SparkFun_RV8803.h#L129-L138
        let mut buf = [0_u8; 8];
        for (i, el) in dest.iter().enumerate() {
            // Note: Weekday does not undergo BCD to Decimal conversion.
            if i == 4 {
                buf[i] = *el;
            } else {
                defmt::info!("Raw: {} / BCD to Dec: {}", *el, bcd_to_dec(*el));
                buf[i] = bcd_to_dec(*el);
            }
        }

        dest.copy_from_slice(&buf[..dest.len()]);

        Ok(true)
    }

    /// Write a single bit to the specified register
    pub fn write_bit(&mut self, reg_addr: u8, bit_addr: u8, bit_to_write: bool) -> Result<bool, E> {
        let mut value = 0;
        if let Ok(reg_value) = self.bus.read_register_by_addr(reg_addr) {
            value = reg_value;
        }

        value &= !(1 << bit_addr);
        value |= u8::from(bit_to_write) << bit_addr;

        self.bus.write_register_by_addr(reg_addr, value)?;

        Ok(true)
    }

    /// Read seconds from the RTC clock
    pub fn read_seconds(&mut self) -> Result<u8, E> {
        let secs = self.bus.read_register(Register::Seconds)?;

        Ok(bcd_to_dec(secs))
    }

    /// Read the year from the RTC clock
    pub fn read_year(&mut self) -> Result<u8, E> {
        let year = self.bus.read_register(Register::Year)?;

        Ok(bcd_to_dec(year))
    }

    /// Set the year and update the RTC clock
    pub fn set_year(&mut self, year: u16) -> Result<u8, E>
    where
        E: core::convert::From<core::num::TryFromIntError>,
    {
        let year = dec_to_bcd(u8::try_from(year - 2000)?);

        self.bus.write_register(Register::Year, year)?;

        self.read_year()
    }
}

fn bcd_to_dec(value: u8) -> u8 {
    ((value / 0x10) * 10) + (value % 0x10)
}

fn dec_to_bcd(value: u8) -> u8 {
    ((value / 10) * 0x10) + (value % 10)
}
