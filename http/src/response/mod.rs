use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};

use guilded_model::id::{marker::ServerMarker, Id};
use hyper::body::{self, Bytes};
use hyper::{Body, Response as HyperResponse};

use crate::response::error::{DeserializationError, DeserializationErrorType};

pub mod error;
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

    pub fn bytes(self) -> BytesFuture {
        let body = self.inner.into_body();

        let future = async move {
            body::to_bytes(body)
                .await
                .map_err(|source| DeserializationError {
                    source: Some(Box::new(source)),
                    r#type: DeserializationErrorType::Chunking,
                })
        };

        BytesFuture {
            future: Box::pin(future),
        }
    }
}

pub struct BytesFuture {
    future:
        Pin<Box<dyn Future<Output = Result<Bytes, DeserializationError>> + Send + Sync + 'static>>,
}

impl Future for BytesFuture {
    type Output = Result<Vec<u8>, DeserializationError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Poll::Ready(result) = Pin::new(&mut self.future).poll(cx) {
            Poll::Ready(result.map(|b| b.into_iter().collect()))
        } else {
            Poll::Pending
        }
    }
}
