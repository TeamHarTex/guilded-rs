//! Utilities for the handling of ISO 8601 timestamps.

use std::fmt::{Formatter, Result as FmtResult};

use serde::de::{Deserialize, Error as DeserializeError, Visitor};
use serde::ser::Serialize;
use serde::{Deserializer, Serializer};
use time::{format_description::well_known::Rfc3339, OffsetDateTime, PrimitiveDateTime};

use self::display::TimestampDisplay;

pub mod display;
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

    pub fn display(self) -> TimestampDisplay {
        TimestampDisplay::new(self)
    }
}

impl<'de> Deserialize<'de> for Timestamp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(TimestampDeserializerVisitor)
    }
}

impl Serialize for Timestamp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&self.display())
    }
}

pub(in crate::datetime) struct TimestampDeserializerVisitor;

impl Visitor<'_> for TimestampDeserializerVisitor {
    type Value = Timestamp;

    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("an rfc3339 timestamp")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: DeserializeError,
    {
        Timestamp::parse(value).map_err(DeserializeError::custom)
    }
}
