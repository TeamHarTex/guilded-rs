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
        }
    }
}

impl Error for ChatEmbedFieldValidationError {}

#[derive(Debug)]
#[non_exhaustive]
pub enum ChatEmbedFieldValidationErrorType {
}
