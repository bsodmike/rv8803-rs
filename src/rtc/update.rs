use crate::{error::DriverError, rtc::registers::Register, ClockData};
use core::fmt::Debug;
use embedded_hal::i2c::{I2c, SevenBitAddress};

use super::registers::ClockRegisters;

pub trait Updatable: Debug + Copy + Clone {
    /// Set the date and time.
    ///
    /// # Errors
    ///
    /// Returns a [`DriverError`]
    fn set_datetime<I2C>(
        &mut self,
        i2c: &mut I2C,
        addr: u8,
        cr: &mut ClockRegisters,
        data: &ClockData,
    ) -> Result<(), DriverError<I2C::Error>>
    where
        I2C: I2c<SevenBitAddress>,
        I2C::Error: Into<DriverError<I2C::Error>>;
}

impl Updatable for ClockData {
    fn set_datetime<I2C>(
        &mut self,
        i2c: &mut I2C,
        _addr: u8,
        cu: &mut ClockRegisters,
        data: &ClockData,
    ) -> Result<(), DriverError<I2C::Error>>
    where
        I2C: I2c<SevenBitAddress>,
        I2C::Error: Into<DriverError<I2C::Error>>,
    {
        use crate::models::misc::dec_to_bcd;

        // Stored as BCD values.
        cu.write_register(i2c, Register::Hours, dec_to_bcd(data.hours()))?;
        cu.write_register(i2c, Register::Minutes, dec_to_bcd(data.minutes()))?;
        cu.write_register(i2c, Register::Seconds, dec_to_bcd(data.seconds()))?;
        cu.write_register(i2c, Register::Date, dec_to_bcd(data.day()))?;
        cu.write_register(i2c, Register::Month, dec_to_bcd(data.month()))?;
        cu.write_register(i2c, Register::Year, dec_to_bcd(data.year()))?;

        // Single bit value only
        cu.write_register(i2c, Register::Weekday, data.weekday())?;

        // Set RESET bit to 0 in Control register to prevent seconds getting stuck.
        cu.write_bit(
            i2c,
            Register::Control.address(),
            Register::ControlReset.address(),
            false,
        )?;

        Ok(())
    }
}
