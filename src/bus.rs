use super::Register;
use core::marker::PhantomData;

/// Trait for [`Bus`]
#[allow(clippy::module_name_repetitions)]
pub trait BusTrait {
    /// Bus error.
    type Error;

    /// Read from the `rv8803` chip.
    ///
    /// # Errors
    ///
    /// Will return [`BusTrait::Error`] if the read attempt fails.
    fn read_register(&mut self, register: Register) -> Result<u8, Self::Error>;

    /// Write to the `rv8803` chip.
    ///
    /// # Errors
    ///
    /// Will return [`BusTrait::Error`] if the write attempt fails.
    fn write_register(&mut self, register: Register, value: u8) -> Result<(), Self::Error>;

    /// Read multiple registers
    ///
    /// # Errors
    ///
    /// Will return [`BusTrait::Error`] if the read attempt fails.
    fn read_multiple_registers(
        &mut self,
        addr: u8,
        dest: &mut [u8],
        len: usize,
    ) -> Result<bool, Self::Error>;

    /// Write to register by register address
    ///
    /// # Errors
    ///
    /// Will return [`BusTrait::Error`] if the write attempt fails.
    fn write_register_by_addr(&mut self, reg_addr: u8, value: u8) -> Result<(), Self::Error>;

    /// Read register by register address
    ///
    /// # Errors
    ///
    /// Will return [`BusTrait::Error`] if the read attempt fails.
    fn read_register_by_addr(&mut self, reg_addr: u8) -> Result<u8, Self::Error>;
}

/// This is an I2C bus that implements [`embedded_hal_0_2::blocking::i2c::WriteRead`] and [`embedded_hal_0_2::blocking::i2c::Write`]
#[derive(Debug)]
#[allow(clippy::struct_field_names)]
pub struct Bus<'a, I2C> {
    address: u8,
    bus: I2C,
    _i2c: PhantomData<&'a I2C>,
}

impl<'a, I2C, E> Bus<'a, I2C>
where
    I2C: embedded_hal_0_2::blocking::i2c::WriteRead<Error = E>
        + embedded_hal_0_2::blocking::i2c::Write<Error = E>,
    Bus<'a, I2C>: BusTrait<Error = E>,
{
    /// Creates a new [`Bus`] from an I2C peripheral.
    pub fn new(bus: I2C, address: &u8) -> Self {
        Self {
            bus,
            address: *address,
            _i2c: PhantomData,
        }
    }
}

impl<I2C, E> BusTrait for Bus<'_, I2C>
where
    I2C: embedded_hal_0_2::blocking::i2c::WriteRead<Error = E>
        + embedded_hal_0_2::blocking::i2c::Write<Error = E>,
{
    type Error = E;

    fn read_register(&mut self, register: Register) -> Result<u8, Self::Error> {
        let mut data = [0];
        self.bus
            .write_read(self.address, &[register.address()], &mut data)?;
        Ok(u8::from_le_bytes(data))
    }

    fn write_register(&mut self, register: Register, byte: u8) -> Result<(), E> {
        self.bus.write(self.address, &[register.address(), byte])?;

        Ok(())
    }

    fn read_multiple_registers(
        &mut self,
        addr: u8,
        dest: &mut [u8],
        _len: usize,
    ) -> Result<bool, E> {
        self.bus.write_read(self.address, &[addr], dest)?;

        Ok(true)
    }

    fn write_register_by_addr(&mut self, reg_addr: u8, byte: u8) -> Result<(), Self::Error> {
        self.bus.write(self.address, &[reg_addr, byte])?;

        Ok(())
    }

    fn read_register_by_addr(&mut self, reg_addr: u8) -> Result<u8, Self::Error> {
        let mut data = [0];
        self.bus.write_read(self.address, &[reg_addr], &mut data)?;
        Ok(u8::from_le_bytes(data))
    }
}
