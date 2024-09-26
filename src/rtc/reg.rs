use crate::error::DriverError;
use core::fmt::Debug;
use embedded_hal::i2c::{I2c, SevenBitAddress};

use super::AddressingMode;

#[derive(Debug, Copy, Clone)]
pub struct Register {
    /// points to a specific register in the sensor
    ptr: u8,
    /// register contents, either 1 or 2 bytes
    buf: [u8; 2],
    /// actual register size in bytes, either 1 or 2
    len: u8,
}

impl Register {
    #[allow(dead_code)]
    #[allow(clippy::manual_assert)]
    pub fn new(ptr: u8, len: u8) -> Self {
        if len > 2 {
            panic!("length > 2")
        }

        let buf = [0u8, 0];
        Register { ptr, buf, len }
    }

    pub fn get_buf(&self) -> &[u8] {
        &self.buf[0..self.len as usize]
    }

    pub fn set_buf(&mut self, val: [u8; 2]) {
        self.buf = val;
    }

    pub fn get_ptr(self) -> u8 {
        self.ptr
    }

    pub fn get_len(self) -> u8 {
        self.len
    }
}

/// trait for a register that can be read from an i2c device
pub trait Read: Debug + Copy + Clone {
    fn read_from_device<I2C>(
        &mut self,
        i2c: &mut I2C,
        addr: u8,
    ) -> Result<(), DriverError<I2C::Error>>
    where
        I2C: I2c<SevenBitAddress>,
        I2C::Error: Into<DriverError<I2C::Error>>;
}

impl Read for Register {
    fn read_from_device<I2C>(
        &mut self,
        i2c: &mut I2C,
        addr: u8,
    ) -> Result<(), DriverError<I2C::Error>>
    where
        I2C: I2c<SevenBitAddress>,
        I2C::Error: Into<DriverError<I2C::Error>>,
    {
        let mut buf = [0u8; 2];
        i2c.write_read(
            addr,
            &[self.get_ptr()],
            &mut buf[0..self.get_len() as usize],
        )?;
        self.set_buf(buf);
        Ok(())
    }
}

/// trait for a register that can be written to an i2c device
pub trait Write: Read {
    fn write_to_device<I2C, A>(
        &self,
        i2c: &mut I2C,
        addr: u8,
    ) -> Result<(), DriverError<I2C::Error>>
    where
        I2C: I2c<A::Mode>,
        I2C::Error: Into<DriverError<I2C::Error>>,
        A: AddressingMode<Mode = SevenBitAddress> + embedded_hal::i2c::AddressMode;
}

impl Write for Register {
    fn write_to_device<I2C, A>(
        &self,
        i2c: &mut I2C,
        addr: u8,
    ) -> Result<(), DriverError<I2C::Error>>
    where
        I2C: I2c<A::Mode>,
        I2C::Error: Into<DriverError<I2C::Error>>,
        A: AddressingMode<Mode = SevenBitAddress> + embedded_hal::i2c::AddressMode,
    {
        // reg ptr + 1 or 2 bytes
        let mut buf = [self.get_ptr(); 3];
        for (i, item) in self.get_buf().iter().enumerate() {
            buf[i + 1] = *item;
        }
        Ok(i2c.write(addr, &buf[0..self.get_len() as usize])?)
    }
}
