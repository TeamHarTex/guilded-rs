//! Utilities for the handling of ISO 8601 timestamps.

use std::fmt::{Formatter, Result as FmtResult};

use serde::de::{Deserialize, Error as DeserializeError,  Visitor};
use time::{format_description::well_known::Rfc3339, OffsetDateTime, PrimitiveDateTime};

pub mod error;

/// Representation of a UNIX timestamp.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Timestamp(PrimitiveDateTime);

impl Timestamp {
    /// Parse a timestamp from an ISO 8601 datetime string.
    pub fn parse(datetime: &str) -> Result<Self, error::TimestampParseError> {
        const LENGTH: usize = "2022-01-01T00:00:00Z".len();

        if datetime.len() < LENGTH {
            return Err(error::TimestampParseError {
                kind: error::TimestampParseErrorKind::Format,
                source: None,
            });
        }

        OffsetDateTime::parse(datetime, &Rfc3339)
            .map(|offset| PrimitiveDateTime::new(offset.date(), offset.time()))
            .map(Self)
            .map_err(error::TimestampParseError::from_parse)
    }
}

impl<'de> Deserialize<'de> for Timestamp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        deserializer.deserialize_any(TimestampDeserializerVisitor)
    }
}

pub(in crate::datetime) struct TimestampDeserializerVisitor;

impl Visitor<'_> for TimestampDeserializerVisitor {
    type Value = Timestamp;

    fn expecting(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        formatter.write_str("an rfc3339 timestamp")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: DeserializeError, {
        Timestamp::parse(value).map_err(DeserializeError::custom)
    }
}
