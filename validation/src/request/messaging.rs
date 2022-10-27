use std::error::Error;
use std::fmt::{self, Display, Formatter};

use crate::request::RequestValidationError;
use crate::ValidationResult;

#[derive(Debug)]
pub struct RequestMessagingValidationError {
    r#type: RequestMessagingValidationErrorType,
}

impl Display for RequestMessagingValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.r#type {
            RequestMessagingValidationErrorType::InvalidReadManyLimit => {
                f.write_str("invalid read many limit")
            }
        }
    }
}

impl Error for RequestMessagingValidationError {}

#[derive(Debug)]
#[non_exhaustive]
pub enum RequestMessagingValidationErrorType {
    InvalidReadManyLimit,
}

pub const READ_MANY_LIMIT_MIN: u64 = 1;
pub const READ_MANY_LIMIT_MAX: u64 = 100;

pub fn validate_read_many_limit(limit: u64) -> ValidationResult<RequestValidationError> {
    if !(READ_MANY_LIMIT_MIN..=READ_MANY_LIMIT_MAX).contains(&limit) {
        return Err(RequestValidationError::Messaging(
            RequestMessagingValidationError {
                r#type: RequestMessagingValidationErrorType::InvalidReadManyLimit,
            },
        ));
    }

    Ok(())
}
