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

use crate::error::{Error, ErrorType};
use crate::response::Response;

pub struct ResponseFuture<T> {
    phantom: PhantomData<T>,
    stage: ResponseStage,
}

enum ResponsePoll<T> {
    Advance(ResponseStage),
    Pending(ResponseStage),
    Ready(Output<T>),
}

enum ResponseStage {
    Sending(Sending),
}

struct Sending {
    future: Pin<Box<Timeout<HyperResponseFuture>>>,
    guild_id: Option<Id<ServerMarker>>,
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
            Poll::Pending => return ResponsePoll::Pending(ResponseStage::Sending(self))
        };

        if response.status() == StatusCode::UNAUTHORIZED {
            if let Some(unauthorized) = self.unauthorized {
                unauthorized.store(true, Ordering::Relaxed);
            }
        }

        let future = async {
            Response::<()>::new(response)
                .bytes()
        };
    }
}

type Output<T> = Result<Response<T>, Error>;
