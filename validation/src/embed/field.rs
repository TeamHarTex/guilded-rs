use std::error::Error;
use std::fmt::{self, Display, Formatter};

use crate::ValidationResult;

#[derive(Debug)]
pub struct ChatEmbedFieldValidationError {
    r#type: ChatEmbedFieldValidationErrorType,
}

impl Display for ChatEmbedFieldValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.r#type {
            ChatEmbedFieldValidationErrorType::InvalidNameLength => {
                f.write_str("invalid length of embed field name")
            }
            ChatEmbedFieldValidationErrorType::InvalidValueLength => {
                f.write_str("invalid length of embed field value")
            }
        }
    }
}

impl Error for ChatEmbedFieldValidationError {}

#[derive(Debug)]
#[non_exhaustive]
pub enum ChatEmbedFieldValidationErrorType {
    InvalidNameLength,
    InvalidValueLength,
}

pub const EMBED_FIELD_NAME_MAX_LENGTH: usize = 256;
pub const EMBED_FIELD_VALUE_MAX_LENGTH: usize = 1024;

pub fn validate_field_name_length(
    name: impl AsRef<str>,
) -> ValidationResult<ChatEmbedFieldValidationError> {
    let length = name.as_ref().chars().count();

    if length <= EMBED_FIELD_NAME_MAX_LENGTH {
        return Ok(());
    }

    Err(ChatEmbedFieldValidationError {
        r#type: ChatEmbedFieldValidationErrorType::InvalidNameLength,
    })
}

pub fn validate_field_value_length(
    value: impl AsRef<str>,
) -> ValidationResult<ChatEmbedFieldValidationError> {
    let length = value.as_ref().chars().count();

    if length <= EMBED_FIELD_VALUE_MAX_LENGTH {
        return Ok(());
    }

    Err(ChatEmbedFieldValidationError {
        r#type: ChatEmbedFieldValidationErrorType::InvalidValueLength,
    })
}
