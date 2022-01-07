//! A resource ID with type-safe markers.

use std::{
    any,
    fmt::{
        Debug,
        Display,
        Formatter,
        Result as FmtResult
    },
    marker::PhantomData
};

use uuid::Uuid;

pub mod marker;

/// The ID of a resource.
#[derive(Clone)]
pub struct Id<M> {
    phantom: PhantomData<M>,
    value: IdValue
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
        }
        else {
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

/// Inner value of an ID; can either be a UUID or a 8-character unique ID.
#[derive(Clone)]
pub enum IdValue {
    Uuid(Uuid),
    EightCharId(String)
}

impl Debug for IdValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Uuid(uuid) => Display::fmt(uuid, f),
            Self::EightCharId(id) => Display::fmt(id, f)
        }
    }
}
