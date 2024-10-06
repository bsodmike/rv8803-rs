use crate::{models::Year, ClockData};

/// Loggable newtype for [`ClockData`]
#[derive(Debug, Copy, Clone, Default)]
pub struct LoggableClockData {
    data: ClockData,
    /// Current century
    century: Year,
}

impl LoggableClockData {
    /// Creates a [`LoggableClockData`]
    pub fn new(data: ClockData) -> Self {
        Self {
            data,
            century: Year::default(),
        }
    }

    /// Set the century. This is mainly used for presentational purposes.
    pub fn set_century(&mut self, value: Year) {
        self.century = value
    }

    /// Get the clock data.
    pub fn data(&self) -> ClockData {
        self.data
    }

    /// Get the century.
    pub fn century(&self) -> Year {
        self.century
    }
}
