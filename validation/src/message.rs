use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct MessageValidationError {
    r#type: MessageValidationErrorType,
}

impl Display for MessageValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.r#type {
            MessageValidationErrorType::InvalidContentLength => {
                f.write_str("invalid length of message content")
            }
        }
    }
}

impl Error for MessageValidationError {}

#[derive(Debug)]
#[non_exhaustive]
pub enum MessageValidationErrorType {
    InvalidContentLength,
}

pub const MESSAGE_CONTENT_MIN_LENGTH: usize = 1;
pub const MESSAGE_CONTENT_MAX_LENGTH: usize = 4000;

pub fn validate_content_length(name: impl AsRef<str>) -> Result<(), MessageValidationError> {
    let length = name.as_ref().chars().count();

    if (MESSAGE_CONTENT_MIN_LENGTH..=MESSAGE_CONTENT_MAX_LENGTH).contains(&length) {
        return Ok(());
    }

    Err(MessageValidationError {
        r#type: MessageValidationErrorType::InvalidContentLength,
    })
}
