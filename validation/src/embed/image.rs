use std::error::Error;
use std::fmt::{self, Display, Formatter};

use crate::ValidationResult;

#[derive(Debug)]
pub struct ChatEmbedImageValidationError {
    r#type: ChatEmbedImageValidationErrorType,
}

impl Display for ChatEmbedImageValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.r#type {
            ChatEmbedImageValidationErrorType::InvalidImageUrlLength => {
                f.write_str("invalid length of embed image url")
            }
        }
    }
}

impl Error for ChatEmbedImageValidationError {}

#[derive(Debug)]
#[non_exhaustive]
pub enum ChatEmbedImageValidationErrorType {
    InvalidImageUrlLength,
}

pub const EMBED_IMAGE_URL_MAX_LENGTH: usize = 1024;

pub fn validate_image_length(
    image: impl AsRef<str>,
) -> ValidationResult<ChatEmbedImageValidationError> {
    let length = image.as_ref().chars().count();

    if length <= EMBED_IMAGE_URL_MAX_LENGTH {
        return Ok(());
    }

    Err(ChatEmbedImageValidationError {
        r#type: ChatEmbedImageValidationErrorType::InvalidImageUrlLength,
    })
}
