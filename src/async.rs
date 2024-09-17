#[cfg(feature = "async")]
#[allow(unused_imports)]
use crate::error::DriverAsyncError;

use crate::{
    error::DriverTransferError,
    models::{Register, TIME_ARRAY_LENGTH},
};

#[derive(Debug)]
pub struct Driver<'a, P> {
    pub periph: &'a mut P,
}

#[allow(dead_code)]
impl<'a, I2C> Driver<'a, I2C>
// where
//     I2C: embedded_hal::i2c::I2c<Error = Box<dyn embedded_hal::i2c::Error>>,
// DriverTransferError<E>: From<Box<dyn embedded_hal::i2c::Error>>,
{
    /// Creates a new `Driver` driver from a I2C peripheral, and an I2C
    /// device address.
    pub fn using_periph(i2c: &'a mut I2C, address: &u8) -> Self {
        Self { periph: i2c }
    }
}

pub mod bus {
    use super::models::Register;
    use core::marker::PhantomData;

    extern crate alloc;
    use alloc::boxed::Box;

    /// Bus trait (named [`BusTrait`]).
    pub trait BusTrait {
        /// Bus error.
        type Error;

        /// Read from the RV8803
        fn read_register(&mut self, register: Register) -> Result<u8, Self::Error>;

        /// Write to the RV8803
        fn write_register(&mut self, register: Register, value: u8) -> Result<(), Self::Error>;

        /// Read multiple registers
        fn read_multiple_registers(
            &mut self,
            addr: u8,
            dest: &mut [u8],
            len: usize,
        ) -> Result<bool, Self::Error>;

        /// Write to register by register address
        fn write_register_by_addr(&mut self, reg_addr: u8, value: u8) -> Result<(), Self::Error>;

        /// Read register by register address
        fn read_register_by_addr(&mut self, reg_addr: u8) -> Result<u8, Self::Error>;
    }

    /// I2C device address.
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
    #[repr(u8)]
    pub enum Address {
        /// Default device address
        Default = 0x32,
    }

    impl Address {
        /// Value of the address variant
        pub fn value(&self) -> u8 {
            *self as u8
        }
    }

    impl From<Address> for u8 {
        fn from(value: Address) -> Self {
            match value {
                Address::Default => Address::Default.value(),
            }
        }
    }

    /// Holds an instance of an i2c bus, where the bus implements the `embedded-hal` traits.
    #[derive(Debug)]
    pub struct Bus<'a, I2C> {
        address: u8,
        bus: I2C,
        _p: PhantomData<&'a I2C>,
    }

    impl<'a, I2C> Bus<'a, I2C>
    where
        I2C: embedded_hal::i2c::I2c<Error = Box<dyn embedded_hal::i2c::Error>>,
        Bus<'a, I2C>: BusTrait<Error = Box<dyn embedded_hal::i2c::Error>>,
    {
        /// Creates a new `BusTrait` from a I2C peripheral, and an I2C
        /// device address.
        pub fn new(bus: I2C, address: Address) -> Self {
            Self {
                bus,
                address: address as u8,
                _p: PhantomData,
            }
        }
    }

    impl<I2C> BusTrait for Bus<'_, I2C>
    where
        I2C: embedded_hal::i2c::I2c<Error = Box<dyn embedded_hal::i2c::Error>>,
    {
        type Error = Box<dyn embedded_hal::i2c::Error>;

        fn read_register(&mut self, register: Register) -> Result<u8, Self::Error> {
            let mut data = [0];
            self.bus
                .write_read(self.address, &[register.address()], &mut data)?;
            Ok(u8::from_le_bytes(data))
        }

        fn write_register(&mut self, register: Register, byte: u8) -> Result<(), Self::Error> {
            self.bus.write(self.address, &[register.address(), byte])?;

            Ok(())
        }

        fn read_multiple_registers(
            &mut self,
            addr: u8,
            dest: &mut [u8],
            _len: usize,
        ) -> Result<bool, Self::Error> {
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
}

pub mod models {
    /// Mapping of all the registers used to operate the RTC module
    #[derive(Clone, Copy)]
    pub enum Register {
        /// Hundreths
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
        /// Control
        Control = 0x1F,
    }

    impl Register {
        /// Read address value, returns as [`u8`]
        pub fn address(&self) -> u8 {
            *self as u8
        }
    }
}
