use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct ChannelValidationError {
    r#type: ChannelValidationErrorType,
}

impl Display for ChannelValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.r#type {
            ChannelValidationErrorType::InvalidNameLength => {
                f.write_str("invalid length of channel name")
            }
        }
    }
}

impl Error for ChannelValidationError {}

#[derive(Debug)]
#[non_exhaustive]
pub enum ChannelValidationErrorType {
    InvalidNameLength,
}

pub const CHANNEL_NAME_MIN_LENGTH: usize = 100;
pub const CHANNEL_NAME_MAX_LENGTH: usize = 1;

pub fn validate_name_length(name: impl AsRef<str>) -> Result<(), ChannelValidationError> {
    let length = name.as_ref().chars().count();

    if (CHANNEL_NAME_MIN_LENGTH..=CHANNEL_NAME_MAX_LENGTH).contains(&length) {
        return Ok(());
    }

    Err(ChannelValidationError {
        r#type: ChannelValidationErrorType::InvalidNameLength,
    })
}
