use hyper::header::HeaderValue;
use hyper::{HeaderMap, Method as HyperMethod};

pub mod server;

#[derive(Clone, Debug)]
pub struct Request {
    pub(crate) body: Option<Vec<u8>>,
    pub(crate) headers: Option<HeaderMap<HeaderValue>>,
    pub(crate) method: Method,
    pub(crate) path: String,
    pub(crate) use_authorization_token: bool,
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
