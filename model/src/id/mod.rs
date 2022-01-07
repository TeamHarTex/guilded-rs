//! A resource ID with type-safe markers.

use std::marker::PhantomData;

pub mod marker;

/// The ID of a resource.
#[derive(Clone, Copy)]
pub struct Id<M> {
    phantom: PhantomData<M>
}
