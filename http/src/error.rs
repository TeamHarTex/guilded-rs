use std::error::Error as StdError;

#[derive(Debug)]
pub struct Error {
    pub(super) source: Option<Box<dyn StdError + Send + Sync>>,
    pub(super) r#type: ErrorType,
}

#[derive(Debug)]
pub enum ErrorType {
    RequestError,
    RequestTimeout,
}
