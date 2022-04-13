//! A builder for building an HTTP Client.

use std::time::Duration;

/// A builder for a `Client`.
#[derive(Debug)]
pub struct ClientBuilder {
    pub(in crate) timeout: Duration,
    pub(in crate) token: Option<Box<str>>,
}

impl ClientBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the timeout threshold.
    #[must_use = "has no effect when not built into a Client"]
    pub const fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Sets the token to use by the Client for authorization.
    #[must_use = "has no effect when not built into a Client"]
    pub fn token(mut self, mut token: String) -> Self {
        if !token.starts_with("Bearer") {
            token.insert_str(0, "Bearer ");
        }

        self.token.replace(token.into_boxed_str());
        self
    }
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(10),
            token: None,
        }
    }
}
