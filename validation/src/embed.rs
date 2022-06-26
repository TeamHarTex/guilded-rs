use std::error::Error;
use std::fmt::{self, Display, Formatter};

use crate::ValidationResult;

#[derive(Debug)]
pub struct EmbedValidationError {
    r#type: EmbedValidationErrorType,
}

impl Display for EmbedValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.r#type {
            EmbedValidationErrorType::InvalidTitleLength => {
                f.write_str("invalid length of embed title")
            }
        }
    }
}

impl Error for EmbedValidationError {}

#[derive(Debug)]
#[non_exhaustive]
pub enum EmbedValidationErrorType {
    InvalidTitleLength,
}

pub const EMBED_TITLE_MAX_LENGTH: usize = 256;

pub fn validate_title_length(title: impl AsRef<str>) -> ValidationResult<EmbedValidationError> {
    let length = title.as_ref().chars().count();

    if length <= EMBED_TITLE_MAX_LENGTH {
        return Ok(());
    }

    Err(EmbedValidationError {
        r#type: EmbedValidationErrorType::InvalidTitleLength,
    })
}
