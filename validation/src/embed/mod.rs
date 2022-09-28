use std::error::Error;
use std::fmt::{self, Display, Formatter};

use crate::ValidationResult;

pub mod author;
pub mod footer;
pub mod image;
pub mod thumbnail;

#[derive(Debug)]
pub struct ChatEmbedValidationError {
    r#type: ChatEmbedValidationErrorType,
}

impl Display for ChatEmbedValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.r#type {
            ChatEmbedValidationErrorType::InvalidColorValue => {
                f.write_str("color value out of range")
            }
            ChatEmbedValidationErrorType::InvalidDescriptionLength => {
                f.write_str("invalid length of embed description")
            }
            ChatEmbedValidationErrorType::InvalidTitleLength => {
                f.write_str("invalid length of embed title")
            }
        }
    }
}

impl Error for ChatEmbedValidationError {}

#[derive(Debug)]
#[non_exhaustive]
pub enum ChatEmbedValidationErrorType {
    InvalidColorValue,
    InvalidDescriptionLength,
    InvalidTitleLength,
}

pub const EMBED_COLOR_BLACK: u32 = 0xFFFFFF;
pub const EMBED_COLOR_WHITE: u32 = 0x000000;
pub const EMBED_DESCRIPTION_MAX_LENGTH: usize = 2048;
pub const EMBED_TITLE_MAX_LENGTH: usize = 256;

pub fn validate_color_range(color: u32) -> ValidationResult<ChatEmbedValidationError> {
    if (EMBED_COLOR_WHITE..=EMBED_COLOR_BLACK).contains(&color) {
        return Ok(());
    }

    Err(ChatEmbedValidationError {
        r#type: ChatEmbedValidationErrorType::InvalidColorValue,
    })
}

pub fn validate_description_length(
    description: impl AsRef<str>,
) -> ValidationResult<ChatEmbedValidationError> {
    let length = description.as_ref().chars().count();

    if length <= EMBED_DESCRIPTION_MAX_LENGTH {
        return Ok(());
    }

    Err(ChatEmbedValidationError {
        r#type: ChatEmbedValidationErrorType::InvalidDescriptionLength,
    })
}

pub fn validate_title_length(title: impl AsRef<str>) -> ValidationResult<ChatEmbedValidationError> {
    let length = title.as_ref().chars().count();

    if length <= EMBED_TITLE_MAX_LENGTH {
        return Ok(());
    }

    Err(ChatEmbedValidationError {
        r#type: ChatEmbedValidationErrorType::InvalidTitleLength,
    })
}
