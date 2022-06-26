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
            EmbedValidationErrorType::InvalidDescriptionLength => {
                f.write_str("invalid length of embed description")
            }
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
    InvalidDescriptionLength,
    InvalidTitleLength,
}

pub const EMBED_DESCRIPTION_MAX_LENGTH: usize = 2048;
pub const EMBED_TITLE_MAX_LENGTH: usize = 256;

pub fn validate_description_length(
    description: impl AsRef<str>,
) -> ValidationResult<EmbedValidationError> {
    let length = description.as_ref().chars().count();

    if length <= EMBED_DESCRIPTION_MAX_LENGTH {
        return Ok(());
    }

    Err(EmbedValidationError {
        r#type: EmbedValidationErrorType::InvalidDescriptionLength,
    })
}

pub fn validate_title_length(title: impl AsRef<str>) -> ValidationResult<EmbedValidationError> {
    let length = title.as_ref().chars().count();

    if length <= EMBED_TITLE_MAX_LENGTH {
        return Ok(());
    }

    Err(EmbedValidationError {
        r#type: EmbedValidationErrorType::InvalidTitleLength,
    })
}
