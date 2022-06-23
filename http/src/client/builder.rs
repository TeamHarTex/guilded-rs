//! A builder for building an HTTP Client.

use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::time::Duration;

use hyper::client::Client as Hyper;

use crate::client::connector;
use crate::client::Client;

#[derive(Debug)]
pub struct ClientBuilder {
    pub(crate) remember_invalid_token: bool,
    pub(crate) timeout: Duration,
    pub(crate) token: Option<Box<str>>,
    pub(crate) use_http: bool,
}

impl ClientBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Client {
        let connector = connector::build();
        let http = Hyper::builder().build(connector);
        let token_invalidated = if self.remember_invalid_token {
            Some(Arc::new(AtomicBool::new(false)))
        } else {
            None
        };

        Client {
            http,
            timeout: self.timeout,
            token: self.token,
            token_invalidated,
            use_http: self.use_http,
        }
    }

    #[must_use = "has no effect when not built into a Client"]
    pub const fn remember_invalid_token(mut self, remember_invalid_token: bool) -> Self {
        self.remember_invalid_token = remember_invalid_token;
        self
    }

    #[must_use = "has no effect when not built into a Client"]
    pub const fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    #[must_use = "has no effect when not built into a Client"]
    pub fn token(mut self, mut token: String) -> Self {
        if !token.starts_with("Bearer") {
            token.insert_str(0, "Bearer ");
        }

        self.token.replace(token.into_boxed_str());
        self
    }

    #[must_use = "has no effect when not built into a Client"]
    pub const fn use_http(mut self, use_http: bool) -> Self {
        self.use_http = use_http;
        self
    }
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self {
            remember_invalid_token: true,
            timeout: Duration::from_secs(10),
            token: None,
            use_http: false,
        }
    }
}
