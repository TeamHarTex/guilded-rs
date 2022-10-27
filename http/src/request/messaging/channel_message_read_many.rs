use guilded_model::datetime::Timestamp;
use guilded_model::id::{marker::ChannelMarker, Id};
use guilded_model::messaging::ChatMessage;
use guilded_validation::request::{messaging, RequestValidationError};

use crate::client::Client;
use crate::error::Error;
use crate::request::Request;
use crate::response::future::ResponseFuture;
use crate::route::Route;

#[must_use = "requests must be configured and executed"]
pub struct ChannelMessageReadMany<'a> {
    after: Option<Timestamp>,
    before: Option<Timestamp>,
    client: &'a Client,
    channel_id: Id<ChannelMarker>,
    include_private: Option<bool>,
    limit: Option<u64>,
}

impl<'a> ChannelMessageReadMany<'a> {
    pub(crate) fn new(client: &'a Client, channel_id: Id<ChannelMarker>) -> Self {
        Self {
            after: None,
            before: None,
            client,
            channel_id,
            include_private: None,
            limit: None,
        }
    }

    pub fn after(mut self, after: Timestamp) -> Self {
        self.after.replace(after);
        self
    }

    pub fn before(mut self, before: Timestamp) -> Self {
        self.before.replace(before);
        self
    }

    pub fn include_private(mut self, include_private: bool) -> Self {
        self.include_private.replace(include_private);
        self
    }

    pub fn limit(mut self, limit: u64) -> Result<Self, RequestValidationError> {
        messaging::validate_read_many_limit(limit)?;

        self.limit.replace(limit);
        Ok(self)
    }

    pub fn finish(self) -> ResponseFuture<Vec<ChatMessage>> {
        let client = self.client;

        match self.try_into() {
            Ok(request) => client.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryInto<Request> for ChannelMessageReadMany<'_> {
    type Error = Error;

    fn try_into(self) -> Result<Request, Self::Error> {
        Ok(Request::builder(&Route::ChannelMessageReadMany {
            after: self.after,
            before: self.before,
            channel_id: &self.channel_id.value().unwrap_alphanumeric(),
            include_private: self.include_private,
            limit: self.limit,
        })
        .build())
    }
}
