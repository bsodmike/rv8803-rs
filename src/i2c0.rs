use super::Register;

/// I2C device address.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
#[repr(u8)]
pub enum Address {
    /// Default device address
    Default = 0x32,
}

impl Into<u8> for Address {
    fn into(self) -> u8 {
        self::Address::Default as u8
    }
}

/// RV8803 bus.
#[derive(Debug)]
pub struct Rv8803Bus<I2C> {
    address: u8,
    bus: I2C,
}

impl<I2C, E> Rv8803Bus<I2C>
where
    I2C: embedded_hal_0_2::blocking::i2c::Write<Error = E>
        + embedded_hal_0_2::blocking::i2c::WriteRead<Error = E>,
{
    /// Creates a new `Rv8803Bus` from a I2C peripheral, and an I2C
    /// device address.
    #[inline]
    pub fn new(bus: I2C, address: Address) -> Self {
        Self {
            bus,
            address: address as u8,
        }
    }
}

impl<I2C, E> crate::Rv8803Bus for Rv8803Bus<I2C>
where
    I2C: embedded_hal_0_2::blocking::i2c::Write<Error = E>
        + embedded_hal_0_2::blocking::i2c::WriteRead<Error = E>,
{
    type Error = E;

    fn read_register(&mut self, register: Register) -> Result<u8, Self::Error> {
        let mut data = [0];
        self.bus
            .write_read(self.address as u8, &[register.address()], &mut data)?;
        Ok(u8::from_le_bytes(data))
    }

    fn write_register(&mut self, register: Register, byte: u8) -> Result<(), E> {
        self.bus
            .write(self.address as u8, &[register.address(), byte])?;

        Ok(())
    }

    fn read_multiple_registers(
        &mut self,
        addr: u8,
        dest: &mut [u8],
        _len: usize,
    ) -> Result<bool, E> {
        self.bus.write_read(self.address as u8, &[addr], dest)?;

        Ok(true)
    }

    fn write_register_by_addr(&mut self, reg_addr: u8, value: u8) -> Result<(), Self::Error> {
        let byte = value as u8;
        self.bus.write(self.address as u8, &[reg_addr, byte])?;

        Ok(())
    }

    fn read_register_by_addr(&mut self, reg_addr: u8) -> Result<u8, Self::Error> {
        let mut data = [0];
        self.bus
            .write_read(self.address as u8, &[reg_addr], &mut data)?;
        Ok(u8::from_le_bytes(data))
    }
}
