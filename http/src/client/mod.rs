//! An HTTP Client for communicating with the Guilded Bot API.

use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::time::Duration;

use guilded_model::channel::ServerChannelType;
use guilded_model::id::{
    marker::{ChannelMarker, ServerMarker},
    Id,
};
use guilded_validation::channel::ChannelValidationError;
use hyper::client::Client as Hyper;
use hyper::header::{HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE};
use hyper::{Body, Request as HyperRequest};
use tokio::time;

use crate::client::builder::ClientBuilder;
use crate::client::connector::Connector;
use crate::error::{Error, ErrorType};
use crate::request::server::server_channel_create::ServerChannelCreate;
use crate::request::server::server_channel_delete::ServerChannelDelete;
use crate::request::server::server_channel_read::ServerChannelRead;
use crate::request::server::server_channel_update::ServerChannelUpdate;
use crate::request::server::server_read::ServerRead;
use crate::request::{Method, Request};
use crate::response::future::ResponseFuture;
use crate::API_VERSION;
pub mod builder;
pub mod connector;

#[derive(Debug)]
pub struct Client {
    http: Hyper<Connector>,
    timeout: Duration,
    token: Option<Box<str>>,
    token_invalidated: Option<Arc<AtomicBool>>,
    use_http: bool,
}

impl Client {
    pub fn new(token: String) -> Self {
        ClientBuilder::new().token(token).build()
    }

    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    pub fn token(&self) -> Option<&str> {
        self.token.as_deref()
    }

    pub fn server_channel_create<'a>(
        &'a self,
        name: &'a str,
        r#type: ServerChannelType,
    ) -> Result<ServerChannelCreate<'a>, ChannelValidationError> {
        ServerChannelCreate::new(self, name, r#type)
    }

    pub fn server_channel_delete(&self, channel_id: Id<ChannelMarker>) -> ServerChannelDelete {
        ServerChannelDelete::new(self, channel_id)
    }

    pub fn server_channel_read(&self, channel_id: Id<ChannelMarker>) -> ServerChannelRead {
        ServerChannelRead::new(self, channel_id)
    }

    pub fn server_channel_update(&self, channel_id: Id<ChannelMarker>) -> ServerChannelUpdate {
        ServerChannelUpdate::new(self, channel_id)
    }

    pub fn server_read(&self, server_id: Id<ServerMarker>) -> ServerRead {
        ServerRead::new(self, server_id)
    }

    pub fn request<T>(&self, request: Request) -> ResponseFuture<T> {
        match self.try_request(request) {
            Ok(future) => future,
            Err(source) => ResponseFuture::error(source),
        }
    }

    fn try_request<T>(&self, request: Request) -> Result<ResponseFuture<T>, Error> {
        let Request {
            body,
            headers: request_headers,
            method,
            path,
            use_authorization_token,
        } = request;

        let protocol = if self.use_http { "http" } else { "https" };
        let uri = format!("{protocol}://www.guilded.gg/api/v{API_VERSION}/{path}");

        let mut builder = HyperRequest::builder()
            .method(method.as_hyper_method())
            .uri(&uri);

        if use_authorization_token {
            if let Some(token) = &self.token {
                let value = HeaderValue::from_str(token).map_err(|source| Error {
                    source: Some(Box::new(source)),
                    r#type: ErrorType::HttpHeaderCreation {
                        name: AUTHORIZATION.to_string(),
                    },
                })?;

                if let Some(headers) = builder.headers_mut() {
                    headers.insert(AUTHORIZATION, value);
                }
            }
        }

        if let Some(headers) = builder.headers_mut() {
            if let Some(bytes) = &body {
                headers.insert(CONTENT_LENGTH, HeaderValue::from(bytes.len()));
                headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
            } else if matches!(method, Method::Put | Method::Post | Method::Patch) {
                headers.insert(CONTENT_LENGTH, HeaderValue::from(0));
            }

            headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

            if let Some(request_headers) = request_headers {
                for (maybe_name, value) in request_headers {
                    if let Some(name) = maybe_name {
                        headers.insert(name, value);
                    }
                }
            }
        }

        let try_request = if let Some(bytes) = body {
            builder.body(Body::from(bytes))
        } else {
            builder.body(Body::empty())
        };

        let future = self.http.request(try_request.map_err(|source| Error {
            source: Some(Box::new(source)),
            r#type: ErrorType::HttpRequestBuild,
        })?);

        let unauthorized = use_authorization_token
            .then(|| self.token_invalidated.clone())
            .flatten();

        Ok(ResponseFuture::new(
            Box::pin(time::timeout(self.timeout, future)),
            unauthorized,
        ))
    }
}
