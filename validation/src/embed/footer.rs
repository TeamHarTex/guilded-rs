use std::error::Error;
use std::fmt::{self, Display, Formatter};

use crate::ValidationResult;

#[derive(Debug)]
pub struct EmbedFooterValidationError {
    r#type: EmbedFooterValidationErrorType,
}

impl Display for EmbedFooterValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.r#type {
            EmbedFooterValidationErrorType::InvalidTextLength => {
                f.write_str("invalid length of embed footer text")
            }
        }
    }
}

impl Error for EmbedFooterValidationError {}

#[derive(Debug)]
#[non_exhaustive]
pub enum EmbedFooterValidationErrorType {
    InvalidTextLength,
}

pub const EMBED_FOOTER_TEXT_MAX_LENGTH: usize = 2048;

pub fn validate_footer_text_length(
    footer_text: impl AsRef<str>,
) -> ValidationResult<EmbedFooterValidationError> {
    let length = footer_text.as_ref().chars().count();

    if length <= EMBED_FOOTER_TEXT_MAX_LENGTH {
        return Ok(());
    }

    Err(EmbedFooterValidationError {
        r#type: EmbedFooterValidationErrorType::InvalidTextLength,
    })
}
