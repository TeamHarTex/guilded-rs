//! An HTTP Client for communicating with the Guilded Bot API.

use std::time::Duration;

use hyper::client::Client as Hyper;

use crate::client::builder::ClientBuilder;
use crate::client::connector::Connector;
use crate::error::Error;
use crate::request::Request;
use crate::response::future::ResponseFuture;

pub mod builder;
pub mod connector;

#[derive(Debug)]
pub struct Client {
    http: Hyper<Connector>,
    timeout: Duration,
    token: Option<Box<str>>,
}

impl Client {
    pub fn new(token: String) -> Self {
        ClientBuilder::new().token(token).build()
    }

    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    pub fn token(&self) -> Option<&str> {
        self.token.as_deref()
    }

    pub fn request<T>(&self, request: Request) -> ResponseFuture<T> {
        match self.try_request(request) {
            Ok(future) => future,
            Err(source) => ResponseFuture::error(source),
        }
    }

    fn try_request<T>(&self, _: Request) -> Result<ResponseFuture<T>, Error> {
        todo!()
    }
}
