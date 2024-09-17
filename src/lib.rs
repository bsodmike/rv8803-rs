//! RTC clock driver for the `rv8803` chip over I2C.
//!
//! Latest implementation supports `blocking` transfer, with plans to support `Async` in the future via `embedded-hal` v1.0.
#![no_std]
#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(feature = "async", allow(incomplete_features))]
#![cfg_attr(docsrs, feature(doc_cfg), feature(doc_auto_cfg))]
#![feature(error_generic_member_access)]
#![feature(trivial_bounds)]
#![allow(unexpected_cfgs)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub use embedded_hal_0_2;

/// An I2C bus, allowing communications over an I2C peripheral.
pub mod bus;

/// Underlying driver.
pub(crate) mod driver;

pub(crate) mod error;

/// Models
pub(crate) mod models;

pub(crate) mod r#async;

/// Driver for the `rv8803` rtc chip.
pub mod rtc;

/// Experimental Re-exports
// #[cfg(feature = "experimental")]
pub mod experimental {
    pub use crate::error::{DriverTransferError, Error};
    pub use crate::r#async::Driver;
}

/// Re-exports
pub mod prelude {
    #[cfg(feature = "blocking")]
    pub use crate::bus::Bus;

    #[cfg(feature = "async")]
    pub use crate::bus::BusAsync;

    pub use crate::error::{DriverTransferError, Error};
}
