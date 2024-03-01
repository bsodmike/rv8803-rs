//! This example reads the chip ID from a RV8803.
#[cfg(feature = "linux_embedded_hal")]
use linux_embedded_hal::I2cdev;
#[allow(unused_imports)]
use rv8803::models::{CrateError, Rv8803, TIME_ARRAY_LENGTH};

#[cfg(feature = "linux_embedded_hal")]
fn main() -> Result<(), CrateError> {
    let dev = I2cdev::new("/dev/i2c-1");
    let i2c = dev.map_err(CrateError::default_err_with_cause)?;

    let mut rtc: Rv8803<_> =
        Rv8803::from_i2c(i2c, rv8803::bus::Address::Default).expect("Failed to initialize RV8803");

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

#[cfg(not(feature = "linux_embedded_hal"))]
fn main() -> Result<(), CrateError> {
    Ok(())
}
