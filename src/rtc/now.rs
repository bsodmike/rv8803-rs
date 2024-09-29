use crate::formatter::ByteMutWriter;
use crate::models::ClockData;
use crate::{error::DriverError, models::Month, models::Weekday};
use core::fmt::{Debug, Write};
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

fn left_pad<'a>(buf: &'a mut ByteMutWriter<'_>, value: u8) -> &'a str {
    buf.clear();
    write!(buf, "{}{}", if value < 10 { "0" } else { "" }, value).unwrap();

    buf.as_str()
}

impl defmt::Format for ClockData {
    fn format(&self, fmt: defmt::Formatter) {
        let mut buf = [0u8; 2];
        let mut buf = ByteMutWriter::new(&mut buf[..]);
        let hours = left_pad(&mut buf, self.hours);

        let mut buf = [0u8; 2];
        let mut buf = ByteMutWriter::new(&mut buf[..]);
        let minutes = left_pad(&mut buf, self.minutes);

        let mut buf = [0u8; 2];
        let mut buf = ByteMutWriter::new(&mut buf[..]);
        let seconds = left_pad(&mut buf, self.seconds);

        let mut buf = [0u8; 2];
        let mut buf = ByteMutWriter::new(&mut buf[..]);
        let day = left_pad(&mut buf, self.date);

        let month = Month::from(self.month);
        let weekday = Weekday::from(self.weekday);

        defmt::write!(
            fmt,
            "{}:{}:{}, {}, {} {} {}",
            hours,
            minutes,
            seconds,
            weekday,
            day,
            month,
            self.year(),
        );
    }
}
