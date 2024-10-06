use crate::error::DriverError;
use crate::models::ClockData;
use core::fmt::Debug;
use embedded_hal::i2c::{I2c, SevenBitAddress};

/// Trait to read from I2C periph
pub trait Readable: Debug + Copy + Clone {
    /// Fetch the latest date and time.
    ///
    /// # Errors
    ///
    /// Returns a [`DriverError`]
    fn now<I2C>(
        &mut self,
        i2c: &mut I2C,
        addr: u8,
        data: &mut ClockData,
    ) -> Result<(), DriverError<I2C::Error>>
    where
        I2C: I2c<SevenBitAddress>,
        I2C::Error: Into<DriverError<I2C::Error>>;
}

impl Readable for ClockData {
    fn now<I2C>(
        &mut self,
        i2c: &mut I2C,
        addr: u8,
        data: &mut ClockData,
    ) -> Result<(), DriverError<I2C::Error>>
    where
        I2C: I2c<SevenBitAddress>,
        I2C::Error: Into<DriverError<I2C::Error>>,
    {
        use crate::models::misc::bcd_to_dec;

        let mut cregs = super::registers::new(addr);

        let latest = ClockData {
            hundredths: bcd_to_dec(
                cregs.read_register(i2c, super::registers::Register::Hundredths)?,
            ),
            seconds: bcd_to_dec(cregs.read_register(i2c, super::registers::Register::Seconds)?),
            minutes: bcd_to_dec(cregs.read_register(i2c, super::registers::Register::Minutes)?),
            hours: bcd_to_dec(cregs.read_register(i2c, super::registers::Register::Hours)?),
            date: bcd_to_dec(cregs.read_register(i2c, super::registers::Register::Date)?),
            month: bcd_to_dec(cregs.read_register(i2c, super::registers::Register::Month)?),
            year: bcd_to_dec(cregs.read_register(i2c, super::registers::Register::Year)?),

            // Read directly as a byte.
            weekday: cregs.read_register(i2c, super::registers::Register::Weekday)?,
        };

        *data = latest;

        Ok(())
    }
}
