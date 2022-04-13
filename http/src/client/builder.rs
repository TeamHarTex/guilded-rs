//! A builder for building an HTTP Client.

use std::time::Duration;

use hyper::client::Client as Hyper;

use crate::client::connector;
use crate::client::Client;

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

    pub fn build(self) -> Client {
        let connector = connector::build();
        let http = Hyper::builder().build(connector);

        Client {
            http,
            timeout: self.timeout,
            token: self.token,
        }
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
