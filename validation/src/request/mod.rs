use std::error::Error;
use std::fmt::{Display, Formatter};

pub mod messaging;

#[derive(Debug)]
pub enum RequestValidationError {
    Messaging(messaging::RequestMessagingValidationError),
}

impl Display for RequestValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Messaging(error) => Display::fmt(error, f),
        }
    }
}

impl Error for RequestValidationError {}
