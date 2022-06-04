use hyper::header::HeaderValue;
use hyper::{HeaderMap, Method as HyperMethod};

#[derive(Clone, Debug)]
pub struct Request {
    pub(crate) body: Option<Vec<u8>>,
    pub(crate) headers: Option<HeaderMap<HeaderValue>>,
    pub(crate) method: HyperMethod,
    pub(crate) path: String,
    pub(crate) use_authorization_token: bool,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum Method {
    Get,
    Delete,
    Patch,
    Post,
    Put,
}
