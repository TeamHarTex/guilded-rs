use hyper::StatusCode;
use std::error::Error as StdError;

#[derive(Debug)]
pub struct Error {
    pub(super) source: Option<Box<dyn StdError + Send + Sync>>,
    pub(super) r#type: ErrorType,
}

#[derive(Debug)]
pub enum ErrorType {
    ChunkingResponse,
    Parsing { body: Vec<u8> },
    RequestError,
    RequestTimeout,
    Response { body: Vec<u8>, status: StatusCode },
}

pub struct ApiError {
    pub code: String,
    pub message: String,
}
