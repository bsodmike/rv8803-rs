//! This example reads the chip ID from a RV8803.

use i2cdev::linux::LinuxI2CError;
use linux_embedded_hal::I2cdev;
use rv8803_rs::{Rv8803, Rv8803Error};

fn main() -> Result<(), Rv8803Error<LinuxI2CError>> {
    let i2c = I2cdev::new("/dev/i2c-1").map_err(Rv8803Error::I2c)?;

    let _rtc: Rv8803<_> = Rv8803::from_i2c0(i2c, rv8803_rs::i2c0::Address::Default)
        .expect("Failed to initialize RV8803");

    std::thread::sleep(std::time::Duration::from_millis(2));

    // sanity check
    // assert_eq!(rtc.chip_id().expect("Failed to read chip ID"), CHIP_ID);
    // println!("Chip ID ok");

    // println!("sample = {:#?}", sample);

    Ok(())
}
