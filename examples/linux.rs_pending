//! This example accesses the `rv8803` rtc chip over I2C.
#![feature(error_generic_member_access)]

use error::Error;
#[allow(unused_imports)]
use rv8803::rtc::RTClock;

#[cfg(feature = "linux_embedded_hal")]
use linux_embedded_hal::I2cdev;
#[allow(unused_imports)]
use rv8803::rtc::RTClockDirect;

#[cfg(feature = "linux_embedded_hal")]
fn main() -> Result<(), Error> {
    let dev = I2cdev::new("/dev/i2c-1").expect("Unable to unwrap device");
    let device_address: u8 = 0x32;
    let rtc = RTClockDirect::new(dev, &device_address);

    std::thread::sleep(std::time::Duration::from_millis(2));

    let mut time = [0_u8; 8_usize];

    // Fetch time from RTC.
    let update = rtc
        .rtc()
        .update_time(&mut time)
        .expect("Fetched latest time from RTC");
    if !update {
        log::warn!("RTC: Failed reading latest time");
    }

    Ok(())
}

#[cfg(not(feature = "linux_embedded_hal"))]
fn main() -> Result<(), Error> {
    Ok(())
}

mod error {
    /// Boxed error type
    type BoxError = Box<dyn core::error::Error + Send + Sync>;

    #[derive(Debug)]
    pub(crate) struct Error(BoxError);

    impl core::fmt::Display for Error {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "Error: {}", self.0)
        }
    }

    impl core::error::Error for Error {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            self.0.source()
        }

        fn description(&self) -> &str {
            "description() is deprecated; use Display"
        }

        fn cause(&self) -> Option<&dyn std::error::Error> {
            self.source()
        }

        fn provide<'a>(&'a self, _request: &mut std::error::Request<'a>) {}
    }
}
