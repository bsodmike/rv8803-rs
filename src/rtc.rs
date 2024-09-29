use crate::error::DriverError;
use crate::models::ClockData;
use crate::rtc::{address::SlaveAddress, registers as ClockRegisters};
use core::marker::PhantomData;
use embedded_hal::i2c::{I2c, SevenBitAddress};

pub mod address;
pub mod registers;

/// Used to fetch latest readings.
pub mod now;
/// Used to update the rtc clock.
pub mod update;

/// Trait to specify addressing mode.
pub trait AddressingMode {
    /// The addressing mode.
    type Mode;
}

impl AddressingMode for SevenBitAddress {
    type Mode = SevenBitAddress;
}

/// Driver for the `rv8803` rtc chip.
///
/// # Registers
///
/// Refer Page 15: <https://www.microcrystal.com/fileadmin/Media/Products/RTC/App.Manual/RV-8803-C7_App-Manual.pdf>
pub struct Driver<I2C, A> {
    addr: u8,
    i2c: I2C,
    _addr_mode: core::marker::PhantomData<A>,
}

impl<I2C, A> Driver<I2C, A>
where
    I2C: I2c<A::Mode>,
    I2C::Error: Into<DriverError<I2C::Error>>,
    A: AddressingMode<Mode = SevenBitAddress> + embedded_hal::i2c::AddressMode,
{
    /// Creates a new driver from an I2C peripheral.
    pub fn new(i2c: I2C) -> Self {
        Driver {
            addr: SlaveAddress::Default.into(),
            i2c,
            _addr_mode: PhantomData,
        }
    }

    /// Change I2C address
    pub fn set_address(&mut self, addr: SlaveAddress) -> u8 {
        self.addr = addr.into();
        self.addr
    }

    /// release resources
    pub fn free(self) -> I2C {
        self.i2c
    }

    /// Fetch the latest reading from the rtc module.
    ///
    /// # Errors
    ///
    /// Returns a [`DriverError`]
    pub fn now<T>(&mut self, mut clock_data: T) -> Result<ClockData, DriverError<I2C::Error>>
    where
        T: crate::rtc::now::Read,
        I2C: I2c<SevenBitAddress>,
        I2C::Error: Into<DriverError<I2C::Error>>,
    {
        let mut data = crate::prelude::now::new();

        // Associated instance on T, not to be confused with the value data above.
        clock_data.now(&mut self.i2c, self.addr, &mut data)?;

        Ok(data)
    }

    /// Update the rtc module.
    ///
    /// # Errors
    ///
    /// Returns a [`DriverError`]
    pub fn update<T>(
        &mut self,
        mut clock_data: T,
        data: &Option<ClockData>,
    ) -> Result<T, DriverError<I2C::Error>>
    where
        T: crate::rtc::update::Read,
        I2C: I2c<SevenBitAddress>,
        I2C::Error: Into<DriverError<I2C::Error>>,
    {
        let mut cu = ClockRegisters::new(self.addr);

        if let Some(d) = data {
            clock_data.set_datetime(&mut self.i2c, self.addr, &mut cu, d)?;
        }
        Ok(clock_data)
    }
}

/// Async Driver for the `rv8803` rtc chip.
/// *WARNING*: This is in progress, and will be completed in a future release.
#[allow(dead_code)]
pub struct DriverAsync<I2C, A> {
    addr: u8,
    i2c: I2C,
    _addr_mode: core::marker::PhantomData<A>,
}

impl<I2C, A> DriverAsync<I2C, A>
where
    I2C: embedded_hal_async::i2c::I2c<A::Mode>,
    I2C::Error: Into<DriverError<I2C::Error>>,
    A: AddressingMode<Mode = SevenBitAddress> + embedded_hal_async::i2c::AddressMode,
{
    /// Creates a new driver from an I2C peripheral.
    #[allow(dead_code)]
    pub fn new(i2c: I2C) -> Self {
        DriverAsync {
            addr: SlaveAddress::Default.into(),
            i2c,
            _addr_mode: PhantomData,
        }
    }

    /// Fetch the year value.
    ///
    /// # Errors
    ///
    /// Returns a [`DriverError`]
    pub async fn get_year(&mut self, buf: u8) -> Result<(), DriverError<I2C::Error>> {
        self.i2c.write_read(self.addr, &[0x06], &mut [buf]).await?;

        Ok(())
    }
}
