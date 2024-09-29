use crate::error::DriverError;
use core::fmt::Debug;
use embedded_hal::i2c::{I2c, SevenBitAddress};

/// Mapping of all the registers used to operate the RTC module
#[derive(Clone, Copy)]
#[allow(clippy::doc_markdown)]
pub enum Register {
    /// RAM
    Ram = 0x07,
    /// Minutes Alarm
    MinutesAlarm = 0x08,
    /// HoursAlarm
    HoursAlarm = 0x09,
    /// Hundredths
    Hundredths = 0x10,
    /// Seconds
    Seconds = 0x11,
    /// Minutes
    Minutes = 0x12,
    /// Hours
    Hours = 0x13,
    /// Weekday
    Weekday = 0x14,
    /// Date
    Date = 0x15,
    /// Month
    Month = 0x16,
    /// Year
    Year = 0x17,
    /// ControlReset
    ControlReset = 0,
    /// Extension Register
    Extension = 0x1D,
    /// Flag Register
    Flag = 0x1E,
    /// Control Register
    Control = 0x1F,
    /// Offset
    Offset = 0x2C,
    /// Event Control
    Event = 0x2F,
}

impl Register {
    /// Read address value, returns as [`u8`]
    pub fn address(self) -> u8 {
        self as u8
    }
}

#[derive(Debug, Copy, Clone, Default)]
#[allow(clippy::module_name_repetitions)]
pub struct ClockRegisters {
    device_address: u8,
}

pub fn new(address: u8) -> ClockRegisters {
    ClockRegisters {
        device_address: address,
    }
}

impl ClockRegisters {
    /// Write a single bit to the specified register
    pub fn write_bit<I2C>(
        &mut self,
        i2c: &mut I2C,
        reg_addr: u8,
        bit_addr: u8,
        bit_to_write: bool,
    ) -> Result<bool, DriverError<I2C::Error>>
    where
        I2C: I2c<SevenBitAddress>,
        I2C::Error: Into<DriverError<I2C::Error>>,
    {
        let mut value = 0;

        if let Ok(reg_value) = self.read_register_by_addr(i2c, reg_addr) {
            value = reg_value;
        }

        value &= !(1 << bit_addr);
        value |= u8::from(bit_to_write) << bit_addr;

        self.write_register_by_addr(i2c, reg_addr, value)?;

        Ok(true)
    }

    pub fn read_register<I2C>(
        &mut self,
        i2c: &mut I2C,
        register: Register,
    ) -> Result<u8, DriverError<I2C::Error>>
    where
        I2C: I2c<SevenBitAddress>,
        I2C::Error: Into<DriverError<I2C::Error>>,
    {
        let mut data = [0];
        i2c.write_read(self.device_address, &[register.address()], &mut data)?;
        // debug!("data: {:b}", data);

        Ok(u8::from_le_bytes(data))
    }

    pub fn write_register<I2C>(
        &mut self,
        i2c: &mut I2C,
        register: Register,
        byte: u8,
    ) -> Result<(), DriverError<I2C::Error>>
    where
        I2C: I2c<SevenBitAddress>,
        I2C::Error: Into<DriverError<I2C::Error>>,
    {
        i2c.write(self.device_address, &[register.address(), byte])?;
        Ok(())
    }

    pub fn write_register_by_addr<I2C>(
        &mut self,
        i2c: &mut I2C,
        reg_addr: u8,
        byte: u8,
    ) -> Result<(), DriverError<I2C::Error>>
    where
        I2C: I2c<SevenBitAddress>,
        I2C::Error: Into<DriverError<I2C::Error>>,
    {
        i2c.write(self.device_address, &[reg_addr, byte])?;

        Ok(())
    }

    pub fn read_register_by_addr<I2C>(
        &mut self,
        i2c: &mut I2C,
        reg_addr: u8,
    ) -> Result<u8, DriverError<I2C::Error>>
    where
        I2C: I2c<SevenBitAddress>,
        I2C::Error: Into<DriverError<I2C::Error>>,
    {
        let mut data = [0];
        i2c.write_read(self.device_address, &[reg_addr], &mut data)?;
        Ok(u8::from_le_bytes(data))
    }
}
