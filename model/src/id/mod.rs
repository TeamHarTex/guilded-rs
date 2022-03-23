//! A resource ID with type-safe markers.

use std::{
    any,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    marker::PhantomData,
};

use serde::de::{Deserialize, Error as DeError, Visitor};
use serde::ser::Serialize;
use serde::{Deserializer, Serializer};
use uuid::Uuid;

pub mod marker;

/// The ID of a resource.
#[derive(Clone)]
pub struct Id<M> {
    phantom: PhantomData<M>,
    value: IdValue,
}

impl<M> Debug for Id<M> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("Id")?;
        let marker_name = any::type_name::<M>();

        // In most cases, `any::type_name` should return the fully-qualified name of the type.
        if let Some(pos) = marker_name.rfind("::") {
            // If that is the case, we simply extract the last portion of the path of the
            // fully-qualified name (assuming the marker type does not have any generic parameters).
            if let Some(slice) = marker_name.get(pos + 2..) {
                // Get the past portion of the path of the fully-qualified name for the type name
                // to be added to the `Debug` representation.
                f.write_str("<")?;
                f.write_str(slice)?;
                f.write_str(">")?;
            }
        } else {
            // If that is NOT the case, then the type name returned is simply the name of the type.
            // We do no more further processing, simply add the type name to the `Debug`
            // representation.
            f.write_str("<")?;
            f.write_str(marker_name)?;
            f.write_str(">")?;
        }

        f.write_str("(")?;
        Debug::fmt(&self.value, f)?;

        f.write_str(")")
    }
}

impl<'de, M> Deserialize<'de> for Id<M> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(IdDeserializerVisitor::new())
    }
}

impl<M> Serialize for Id<M> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&format_args!("{:?}", self.value))
    }
}

pub(in crate::id) struct IdDeserializerVisitor<M> {
    phantom: PhantomData<M>,
}

impl<M> IdDeserializerVisitor<M> {
    const fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

impl<M> Visitor<'_> for IdDeserializerVisitor<M> {
    type Value = Id<M>;

    fn expecting(&self, f: &mut Formatter) -> FmtResult {
        f.write_str("a valid guilded id")
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: DeError,
    {
        Ok(Id {
            phantom: PhantomData,
            value: IdValue::Int(v),
        })
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: DeError,
    {
        if let Ok(int) = v.parse::<u32>() {
            return self.visit_u32(int);
        }

        if let Ok(uuid) = v.parse::<Uuid>() {
            return Ok(Id {
                phantom: PhantomData,
                value: IdValue::Uuid(uuid),
            });
        }

        Ok(Id {
            phantom: PhantomData,
            value: IdValue::AlphanumericId(v.to_string()),
        })
    }

    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'_>,
    {
        deserializer.deserialize_any(IdDeserializerVisitor::new())
    }
}

/// Inner value of an ID; can either be a UUID, an integer, or an alphanumeric ID.
#[derive(Clone)]
pub enum IdValue {
    /// A unique 8-character ID.
    AlphanumericId(String),
    /// A unique numeric ID.
    Int(u32),
    /// A unique UUID.
    Uuid(Uuid),
}

impl Debug for IdValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::AlphanumericId(id) => Display::fmt(id, f),
            Self::Int(id) => Display::fmt(id, f),
            Self::Uuid(uuid) => Display::fmt(uuid, f),
        }
    }
}
