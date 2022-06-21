use std::marker::PhantomData;

use guilded_model::id::{marker::ServerMarker, Id};
use hyper::{Body, Response as HyperResponse};

pub mod future;

pub struct Response<T> {
    server_id: Option<Id<ServerMarker>>,
    inner: HyperResponse<Body>,
    phantom: PhantomData<T>,
}

impl<T> Response<T> {
    pub(crate) const fn new(inner: HyperResponse<Body>) -> Self {
        Self {
            server_id: None,
            inner,
            phantom: PhantomData,
        }
    }

    pub fn bytes(self) {}
}
