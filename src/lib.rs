//! RV8803 driver with support for I2C.
#![no_std]
#![cfg_attr(feature = "async", allow(incomplete_features))]
#![cfg_attr(docsrs, feature(doc_cfg), feature(doc_auto_cfg))]
#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![feature(error_in_core)]
#![feature(error_generic_member_access)]
#![feature(fn_traits)]
#![feature(unboxed_closures)]

use crate::bus::{Bus, BusTrait};
pub use embedded_hal_0_2;
use log::{debug, warn};
use models::{Register, Rv8803, TIME_ARRAY_LENGTH};

#[cfg(feature = "alloc")]
extern crate alloc;

/// RV8803 I2C bus implementation with embedded-hal version 0.2
pub mod bus;
/// Models
pub mod models;

#[allow(dead_code)]
impl<'a, I2C, E> Rv8803<Bus<'a, I2C>>
where
    I2C: embedded_hal_0_2::blocking::i2c::WriteRead<Error = E>
        + embedded_hal_0_2::blocking::i2c::Write<Error = E>,
    Bus<'a, I2C>: bus::BusTrait<Error = E>,
{
    /// Create a new RV8803 from a [`bus::Bus`].
    pub fn new(bus: Bus<'a, I2C>) -> Result<Self, E> {
        Ok(Self { bus })
    }

    /// Creates a new `Rv8803` driver from a I2C peripheral, and an I2C
    /// device address.
    pub fn from_i2c(i2c: I2C, address: crate::bus::Address) -> Result<Self, E> {
        let bus = crate::bus::Bus::new(i2c, address);

        Self::new(bus)
    }

    /// Set time on the RV8803 module
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
    ) -> Result<bool, E> {
        self.bus
            .write_register(Register::Seconds, dec_to_bcd(sec))?;
        self.bus
            .write_register(Register::Minutes, dec_to_bcd(min))?;
        self.bus.write_register(Register::Hours, dec_to_bcd(hour))?;
        self.bus.write_register(Register::Date, dec_to_bcd(date))?;
        self.bus
            .write_register(Register::Month, dec_to_bcd(month))?;
        self.bus
            .write_register(Register::Year, dec_to_bcd((year - 2000) as u8))?;
        self.bus.write_register(Register::Weekday, weekday)?;

        //Set RESET bit to 0 after setting time to make sure seconds don't get stuck.
        self.write_bit(
            Register::Control.address(),
            Register::ControlReset.address(),
            false,
        )?;

        debug!("rv8803::set_time: updated RTC clock");

        Ok(true)
    }

    /// Fetch time from the RTC clock and store it in the buffer `dest`.
    pub fn update_time(&mut self, dest: &mut [u8]) -> Result<bool, E> {
        if !(self.bus.read_multiple_registers(
            Register::Hundredths.address(),
            dest,
            TIME_ARRAY_LENGTH,
        )?) {
            warn!("update_time: attempt read - fail 1");
            return Ok(false); // Something went wrong
        }

        // If hundredths are at 99 or seconds are at 59, read again to make sure we didn't accidentally skip a second/minute
        if bcd_to_dec(dest[0]) == 99 || bcd_to_dec(dest[1]) == 59 {
            let mut temp_time = [0_u8; TIME_ARRAY_LENGTH];

            debug!("update_time: if hundredths are at 99 or seconds are at 59, read again to make sure we didn't accidentally skip a second/minute / Hundreths: {} / Seconds: {}", bcd_to_dec(dest[0]),bcd_to_dec(dest[1]));

            if !(self.bus.read_multiple_registers(
                Register::Hundredths.address(),
                &mut temp_time,
                TIME_ARRAY_LENGTH,
            )?) {
                warn!("update_time: attempt read - fail 2");
                return Ok(false); // Something went wrong
            };

            // If the reading for hundredths has rolled over, then our new data is correct, otherwise, we can leave the old data.
            if bcd_to_dec(dest[0]) > bcd_to_dec(temp_time[0]) {
                debug!("update_time: the reading for hundredths has rolled over, then our new data is correct. / Hundreths: {} / temp_time[0]: {}",
                bcd_to_dec(dest[0]),
                bcd_to_dec(temp_time[0]));

                for (i, el) in temp_time.iter().enumerate() {
                    dest[i] = *el
                }
            }
        }

        // byte order: https://github.com/sparkfun/SparkFun_RV-8803_Arduino_Library/blob/main/src/SparkFun_RV8803.h#L129-L138
        let mut buf = [0_u8; 8];
        for (i, el) in dest.iter().enumerate() {
            // Note: Weekday does not undergo BCD to Decimal conversion.
            if i != 4 {
                // println!("Raw: {} / BCD to Dec: {}", *el, bcd_to_dec(*el));
                buf[i] = bcd_to_dec(*el)
            } else {
                buf[i] = *el
            }
        }

        dest.copy_from_slice(&buf[..dest.len()]);

        Ok(true)
    }

    /// Write a single bit to the specified register
    pub fn write_bit(&mut self, reg_addr: u8, bit_addr: u8, bit_to_write: bool) -> Result<bool, E> {
        let mut value = 0;
        if let Ok(reg_value) = self.bus.read_register_by_addr(reg_addr) {
            value = reg_value
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
    pub fn set_year(&mut self, year: u16) -> Result<u8, E> {
        let years_since_2000 = (year - 2000) as u8;
        let year = dec_to_bcd(years_since_2000);

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
