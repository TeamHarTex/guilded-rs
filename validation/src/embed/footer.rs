use std::error::Error;
use std::fmt::{self, Display, Formatter};

use crate::ValidationResult;

#[derive(Debug)]
pub struct ChatEmbedFooterValidationError {
    r#type: ChatEmbedFooterValidationErrorType,
}

impl Display for ChatEmbedFooterValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.r#type {
            ChatEmbedFooterValidationErrorType::InvalidIconUrlLength => {
                f.write_str("invalid length of embed footer icon url")
            }
            ChatEmbedFooterValidationErrorType::InvalidTextLength => {
                f.write_str("invalid length of embed footer text")
            }
        }
    }
}

impl Error for ChatEmbedFooterValidationError {}

#[derive(Debug)]
#[non_exhaustive]
pub enum ChatEmbedFooterValidationErrorType {
    InvalidIconUrlLength,
    InvalidTextLength,
}

pub const EMBED_FOOTER_ICON_URL_MAX_LENGTH: usize = 1024;
pub const EMBED_FOOTER_TEXT_MAX_LENGTH: usize = 2048;

pub fn validate_footer_icon_url_length(
    footer_icon_url: impl AsRef<str>,
) -> ValidationResult<ChatEmbedFooterValidationError> {
    let length = footer_icon_url.as_ref().chars().count();

    if length <= EMBED_FOOTER_ICON_URL_MAX_LENGTH {
        return Ok(());
    }

    Err(ChatEmbedFooterValidationError {
        r#type: ChatEmbedFooterValidationErrorType::InvalidIconUrlLength,
    })
}

pub fn validate_footer_text_length(
    footer_text: impl AsRef<str>,
) -> ValidationResult<ChatEmbedFooterValidationError> {
    let length = footer_text.as_ref().chars().count();

    if length <= EMBED_FOOTER_TEXT_MAX_LENGTH {
        return Ok(());
    }

    Err(ChatEmbedFooterValidationError {
        r#type: ChatEmbedFooterValidationErrorType::InvalidTextLength,
    })
}
