pub mod channel;
pub mod embed;
pub mod message;
pub mod request;

pub(crate) type ValidationResult<E> = Result<(), E>;
