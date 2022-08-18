use std::error::Error;
use std::fmt::{self, Display, Formatter};

use crate::ValidationResult;

#[derive(Debug)]
pub struct EmbedThumbnailValidationError {
    r#type: EmbedThumbnailValidationErrorType,
}

impl Display for EmbedThumbnailValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.r#type {
            EmbedThumbnailValidationErrorType::InvalidThumbnailUrlLength => {
                f.write_str("invalid length of embed thumbnail url")
            }
        }
    }
}

impl Error for EmbedThumbnailValidationError {}

#[derive(Debug)]
#[non_exhaustive]
pub enum EmbedThumbnailValidationErrorType {
    InvalidThumbnailUrlLength,
}

pub const EMBED_THUMBNAIL_URL_MAX_LENGTH: usize = 1024;

pub fn validate_thumbnail_length(
    thumbnail_length: impl AsRef<str>,
) -> ValidationResult<EmbedThumbnailValidationError> {
    let length = thumbnail_length.as_ref().chars().count();

    if length <= EMBED_THUMBNAIL_URL_MAX_LENGTH {
        return Ok(());
    }

    Err(EmbedThumbnailValidationError {
        r#type: EmbedThumbnailValidationErrorType::InvalidThumbnailUrlLength,
    })
}
