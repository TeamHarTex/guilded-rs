//! Errors when parsing timestamps.

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

use time::error::Parse;

/// Reason that a timestamp in ISO 8601 format could not be parsed.
#[derive(Debug)]
pub struct TimestampParseError {
    pub kind: TimestampParseErrorKind,
    pub source: Option<Box<dyn Error + Send + Sync>>,
}

impl TimestampParseError {
    /// Create a new error from a `time::error::Error`.
    pub(super) fn from_parse(source: Parse) -> Self {
        Self {
            kind: TimestampParseErrorKind::ParseFailure,
            source: Some(Box::new(source)),
        }
    }
}

impl Display for TimestampParseError {
    fn fmt(&self, formatter: &mut Formatter<'_>) ->FmtResult {
        match self.kind {
            TimestampParseErrorKind::Format => formatter.write_str("invalid iso 8601 string"),
            TimestampParseErrorKind::ParseFailure => formatter.write_str("failed to parse iso 8601 timestamp"),
        }
    }
}

/// Type of `TimestampParseError` that occurred.
#[derive(Debug)]
pub enum TimestampParseErrorKind {
    /// Invalid ISO 8601 format.
    Format,

    /// Failed to parse the timestamp in ISO 8601 format.
    ParseFailure,
}
