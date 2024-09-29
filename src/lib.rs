//! RTC clock driver for the `rv8803` chip over I2C.
#![no_std]
#![forbid(unsafe_code)]
#![warn(missing_docs, dead_code, clippy::unwrap_used)]
#![cfg_attr(docsrs, feature(doc_cfg), feature(doc_auto_cfg))]
// #![deny(unused_imports)]

pub use crate::models::ClockData;
pub use crate::rtc::Driver;
pub use crate::rtc::DriverAsync;

pub(crate) mod error;
#[allow(dead_code)]
pub(crate) mod formatter;
pub(crate) mod models;
pub(crate) mod rtc;

/// Re-exports
pub mod prelude {
    pub use crate::error::DriverError;
    pub use crate::models::{Month, Weekday};
    pub use crate::rtc::address::SlaveAddress;
    pub use crate::rtc::now;
    pub use crate::rtc::update::{self, ClockUpdater};
    pub use crate::rtc::AddressingMode;
}
