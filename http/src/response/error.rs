use std::error::Error;

#[derive(Debug)]
pub struct DeserializationError {
    pub(crate) source: Option<Box<dyn Error + Send + Sync>>,
    pub(crate) r#type: DeserializationErrorType,
}

#[derive(Debug)]
pub enum DeserializationErrorType {
    Chunking,
}
