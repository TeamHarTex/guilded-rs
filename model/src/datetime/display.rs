//! Display implementation for an ISO 8601 timestamp.

use std::fmt::{Display, Formatter, Result as FmtResult};

use super::Timestamp;

pub struct TimestampDisplay(Timestamp);

impl TimestampDisplay {
    pub(in crate::datetime) fn new(timestamp: Timestamp) -> Self {
        Self(timestamp)
    }
}

impl Display for TimestampDisplay {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let year = self.0 .0.year();
        Display::fmt(&(year / 1000), f)?;
        Display::fmt(&(year / 100 % 10), f)?;
        Display::fmt(&(year / 10 % 10), f)?;
        Display::fmt(&(year % 10), f)?;
        f.write_str("-")?;

        let month = self.0 .0.month() as u8;
        Display::fmt(&(month / 10), f)?;
        Display::fmt(&(month % 10), f)?;
        f.write_str("-");

        let day = self.0 .0.day();
        Display::fmt(&(day / 10), f)?;
        Display::fmt(&(day % 10), f)?;
        f.write_str("T")?;

        let hour = self.0 .0.hour();
        Display::fmt(&(hour / 10), f)?;
        Display::fmt(&(hour % 10), f)?;
        f.write_str(":")?;

        let minute = self.0 .0.minute();
        Display::fmt(&(minute / 10 % 6), f)?;
        Display::fmt(&(minute % 10), f)?;
        f.write_str(":")?;

        let second = self.0 .0.second();
        Display::fmt(&(second / 10 % 6), f)?;
        Display::fmt(&(second % 10), f)?;

        let microsecond = self.0 .0.microsecond();
        Display::fmt(&(microsecond / 100_000), f)?;
        Display::fmt(&(microsecond / 10_000 % 10), f)?;
        Display::fmt(&(microsecond / 1_000 % 10), f)?;
        Display::fmt(&(microsecond / 100 % 10), f)?;
        Display::fmt(&(microsecond / 10 % 10), f)?;
        Display::fmt(&(microsecond % 10), f)?;

        f.write_str("+00:00")
    }
}
