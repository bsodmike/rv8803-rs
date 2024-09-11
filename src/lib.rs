//! RTC clock driver for the `rv8803` chip over I2C.
//!
//! Latest implementation supports `blocking` transfer, either directly via an owned I2C peripheral or using a [`shared_bus`].
#![no_std]
#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg), feature(doc_auto_cfg))]
#![feature(error_generic_member_access)]
#![feature(trivial_bounds)]

pub use embedded_hal_0_2;

/// An I2C bus, allowing communications over an I2C peripheral.
pub mod bus;

/// Underlying driver.
pub(crate) mod driver;

pub(crate) mod error;

/// Models
pub(crate) mod models;

/// Driver for the `rv8803` rtc chip.
pub mod rtc;

/// Re-exports
pub mod prelude {
    pub use crate::bus::Bus;
    pub use crate::error::{DriverTransferError, Error};
}
