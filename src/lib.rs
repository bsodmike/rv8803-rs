//! `rv8803` driver for I2C.
#![no_std]
#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(feature = "async", allow(incomplete_features))]
#![cfg_attr(docsrs, feature(doc_cfg), feature(doc_auto_cfg))]
#![feature(error_generic_member_access)]
#![feature(trivial_bounds)]

pub use embedded_hal_0_2;

pub(crate) mod error;

/// A blocking I2C bus, allowing communications over an I2C peripheral.
pub mod bus;

/// Underlying driver.
pub(crate) mod driver;

/// Models
pub(crate) mod models;

/// Driver for the `rv8803` rtc chip.
pub mod rtc;

/// Re-exports
pub mod prelude {
    pub use crate::bus::Bus;
    pub use crate::error::{DriverError, Error};
}
