//! Errors when parsing timestamps.

use std::error::Error;

use time::error::Error as ParseError;

/// Reason that a timestamp in ISO 8601 format could not be parsed.
#[derive(Debug)]
pub struct TimestampParseError {
    pub kind: TimestampParseErrorKind,
    pub source: Option<Box<dyn Error + Send + Sync>>
}

impl TimestampParseError {
    /// Create a new error from a `time::error::Error`.
    pub(super) fn from_parse(source: ParseError) -> Self {
        Self {
            kind: TimestampParseErrorKind::ParseFailure,
            source: Some(Box::new(source))
        }
    }
}

/// Type of `TimestampParseError` that occurred.
#[derive(Debug)]
pub enum TimestampParseErrorKind {
    /// Invalid ISO 8601 format.
    Format,

    /// Failed to parse the timestamp in ISO 8601 format.
    ParseFailure
}
