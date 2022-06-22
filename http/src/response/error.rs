use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct DeserializationError {
    pub(crate) source: Option<Box<dyn Error + Send + Sync>>,
    pub(crate) r#type: DeserializationErrorType,
}

impl Display for DeserializationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.r#type {
            DeserializationErrorType::Chunking => {
                f.write_str("deserialization error: chunking response")
            }
        }
    }
}

impl Error for DeserializationError {}

#[derive(Debug)]
pub enum DeserializationErrorType {
    Chunking,
}
