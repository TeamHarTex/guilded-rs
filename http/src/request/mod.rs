use hyper::header::{HeaderName, HeaderValue};
use hyper::{HeaderMap, Method as HyperMethod};
use serde::Serialize;

use crate::error::{Error, ErrorType};
use crate::route::Route;

pub mod server;

#[derive(Clone, Debug)]
pub struct Request {
    pub(crate) body: Option<Vec<u8>>,
    pub(crate) headers: Option<HeaderMap<HeaderValue>>,
    pub(crate) method: Method,
    pub(crate) path: String,
    pub(crate) use_authorization_token: bool,
}

impl Request {
    pub fn builder(route: &Route<'_>) -> RequestBuilder {
        RequestBuilder::new(route)
    }

    pub fn from_route(route: &Route<'_>) -> Self {
        Self {
            body: None,
            headers: None,
            method: route.method(),
            path: route.to_string(),
            use_authorization_token: true,
        }
    }
}

pub struct RequestBuilder(Request);

impl RequestBuilder {
    pub fn new(route: &Route<'_>) -> Self {
        Self(Request::from_route(route))
    }

    pub fn build(self) -> Request {
        self.0
    }

    pub fn body(mut self, body: Vec<u8>) -> Self {
        self.0.body = Some(body);

        self
    }

    pub fn headers(mut self, iter: impl Iterator<Item = (HeaderName, HeaderValue)>) -> Self {
        self.0.headers.replace(iter.collect());

        self
    }

    pub fn json(self, serialize: &impl Serialize) -> Result<Self, Error> {
        let bytes = crate::json::to_vec(serialize).map_err(|source| Error {
            source: Some(Box::new(source)),
            r#type: ErrorType::Json,
        })?;

        Ok(self.body(bytes))
    }

    pub const fn use_authorization_token(mut self, use_authorization_token: bool) -> Self {
        self.0.use_authorization_token = use_authorization_token;

        self
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum Method {
    Delete,
    Get,
    Patch,
    Post,
    Put,
}

impl Method {
    pub fn as_hyper_method(&self) -> HyperMethod {
        match self {
            Self::Delete => HyperMethod::DELETE,
            Self::Get => HyperMethod::GET,
            Self::Patch => HyperMethod::PATCH,
            Self::Post => HyperMethod::POST,
            Self::Put => HyperMethod::PUT,
        }
    }
}
