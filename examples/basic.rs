//! This example reads the chip ID from a RV8803.

use i2cdev::linux::LinuxI2CError;
use linux_embedded_hal::I2cdev;
use rv8803::{Rv8803, Rv8803Error, TIME_ARRAY_LENGTH};

fn main() -> Result<(), Rv8803Error<LinuxI2CError>> {
    let i2c = I2cdev::new("/dev/i2c-1").map_err(Rv8803Error::I2c)?;

    let mut rtc: Rv8803<_> = Rv8803::from_i2c(i2c, rv8803::bus::Address::Default)
        .expect("Failed to initialize RV8803");

    std::thread::sleep(std::time::Duration::from_millis(2));

    let mut time = [0_u8; TIME_ARRAY_LENGTH];

    // Fetch time from RTC.
    let update = rtc
        .update_time(&mut time)
        .expect("Fetched latest time from RTC");
    if !update {
        log::warn!("RTC: Failed reading latest time");
    }

    Ok(())
}
