use std::error::Error;
use std::fmt::{self, Display, Formatter};

use crate::ValidationResult;

#[derive(Debug)]
pub struct ChatEmbedAuthorValidationError {
    r#type: ChatEmbedAuthorValidationErrorType,
}

impl Display for ChatEmbedAuthorValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.r#type {
            ChatEmbedAuthorValidationErrorType::InvalidIconUrlLength => {
                f.write_str("invalid length of embed author icon url")
            }
            ChatEmbedAuthorValidationErrorType::InvalidNameLength => {
                f.write_str("invalid length of embed author name")
            }
        }
    }
}

impl Error for ChatEmbedAuthorValidationError {}

#[derive(Debug)]
#[non_exhaustive]
pub enum ChatEmbedAuthorValidationErrorType {
    InvalidIconUrlLength,
    InvalidNameLength,
    InvalidUrlLength,
}

pub const EMBED_AUTHOR_ICON_URL_MAX_LENGTH: usize = 1024;
pub const EMBED_AUTHOR_NAME_MAX_LENGTH: usize = 256;
pub const EMBED_AUTHOR_URL_MAX_LENGTH: usize = 1024;

pub fn validate_footer_icon_url_length(
    icon_url: impl AsRef<str>,
) -> ValidationResult<ChatEmbedAuthorValidationError> {
    let length = icon_url.as_ref().chars().count();

    if length <= EMBED_AUTHOR_ICON_URL_MAX_LENGTH {
        return Ok(());
    }

    Err(ChatEmbedAuthorValidationError {
        r#type: ChatEmbedAuthorValidationErrorType::InvalidIconUrlLength,
    })
}

pub fn validate_footer_name_length(
    name: impl AsRef<str>,
) -> ValidationResult<ChatEmbedAuthorValidationError> {
    let length = name.as_ref().chars().count();

    if length <= EMBED_AUTHOR_NAME_MAX_LENGTH {
        return Ok(());
    }

    Err(ChatEmbedAuthorValidationError {
        r#type: ChatEmbedAuthorValidationErrorType::InvalidNameLength,
    })
}


pub fn validate_footer_url_length(
    url: impl AsRef<str>,
) -> ValidationResult<ChatEmbedAuthorValidationError> {
    let length = url.as_ref().chars().count();

    if length <= EMBED_AUTHOR_URL_MAX_LENGTH {
        return Ok(());
    }

    Err(ChatEmbedAuthorValidationError {
        r#type: ChatEmbedAuthorValidationErrorType::InvalidUrlLength,
    })
}
