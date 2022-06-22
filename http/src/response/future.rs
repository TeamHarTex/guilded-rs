use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::task::{Context, Poll};

use guilded_model::id::{marker::ServerMarker, Id};
use hyper::client::ResponseFuture as HyperResponseFuture;
use hyper::StatusCode;
use tokio::time::Timeout;

use crate::error::{ApiError, Error, ErrorType};
use crate::response::Response;

pub struct ResponseFuture<T> {
    phantom: PhantomData<T>,
    stage: ResponseStage,
}

impl<T> ResponseFuture<T> {
    pub(crate) fn new(
        future: Pin<Box<Timeout<HyperResponseFuture>>>,
        unauthorized: Option<Arc<AtomicBool>>,
    ) -> Self {
        Self {
            phantom: PhantomData,
            stage: ResponseStage::Sending(Sending {
                future,
                server_id: None,
                unauthorized,
            }),
        }
    }
}

enum ResponsePoll<T> {
    Advance(ResponseStage),
    Pending(ResponseStage),
    Ready(Output<T>),
}

enum ResponseStage {
    Chunking(Chunking),
    Sending(Sending),
}

struct Chunking {
    future: Pin<Box<dyn Future<Output = Result<Vec<u8>, Error>> + Send + Sync + 'static>>,
    status: StatusCode,
}

impl Chunking {
    fn poll<T>(mut self, cx: &mut Context<'_>) -> ResponsePoll<T> {
        let bytes = match Pin::new(&mut self.future).poll(cx) {
            Poll::Ready(Ok(bytes)) => bytes,
            Poll::Ready(Err(source)) => return ResponsePoll::Ready(Err(source)),
            Poll::Pending => return ResponsePoll::Pending(ResponseStage::Chunking(self)),
        };

        let error = match crate::json::from_bytes::<ApiError>(&bytes) {
            Ok(error) => error,
            Err(source) => {
                return ResponsePoll::Ready(Err(Error {
                    source: Some(Box::new(source)),
                    r#type: ErrorType::Parsing { body: bytes },
                }));
            }
        };

        ResponsePoll::Ready(Err(Error {
            source: None,
            r#type: ErrorType::Response {
                body: bytes,
                error,
                status: self.status,
            },
        }))
    }
}

struct Sending {
    future: Pin<Box<Timeout<HyperResponseFuture>>>,
    server_id: Option<Id<ServerMarker>>,
    unauthorized: Option<Arc<AtomicBool>>,
}

impl Sending {
    fn poll<T>(mut self, cx: &mut Context<'_>) -> ResponsePoll<T> {
        let response = match Pin::new(&mut self.future).poll(cx) {
            Poll::Ready(Ok(Ok(response))) => response,
            Poll::Ready(Ok(Err(error))) => {
                return ResponsePoll::Ready(Err(Error {
                    source: Some(Box::new(error)),
                    r#type: ErrorType::RequestError,
                }));
            }
            Poll::Ready(Err(source)) => {
                return ResponsePoll::Ready(Err(Error {
                    source: Some(Box::new(source)),
                    r#type: ErrorType::RequestTimeout,
                }));
            }
            Poll::Pending => return ResponsePoll::Pending(ResponseStage::Sending(self)),
        };

        if response.status() == StatusCode::UNAUTHORIZED {
            if let Some(unauthorized) = self.unauthorized {
                unauthorized.store(true, Ordering::Relaxed);
            }
        }

        let status = response.status();
        let future = async {
            Response::<()>::new(response)
                .bytes()
                .await
                .map_err(|source| Error {
                    source: Some(Box::new(source)),
                    r#type: ErrorType::ChunkingResponse,
                })
        };

        ResponsePoll::Advance(ResponseStage::Chunking(Chunking {
            future: Box::pin(future),
            status,
        }))
    }
}

type Output<T> = Result<Response<T>, Error>;
