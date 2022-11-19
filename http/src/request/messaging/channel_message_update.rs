use guilded_model::id::{
    marker::{ChannelMarker, MessageMarker},
    Id,
};
use guilded_model::messaging::embed::ChatEmbed;
use guilded_model::messaging::ChatMessage;
use guilded_validation::message::{self, MessageValidationError};
use serde::Serialize;

use crate::client::Client;
use crate::error::Error;
use crate::request::Request;
use crate::response::future::ResponseFuture;
use crate::route::Route;

#[must_use = "requests must be configured and executed"]
pub struct ChannelMessageUpdate<'a> {
    client: &'a Client,
    channel_id: Id<ChannelMarker>,
    fields: ChannelMessageUpdateFields<'a>,
    message_id: Id<MessageMarker>,
}

impl<'a> ChannelMessageUpdate<'a> {
    pub(crate) fn new(
        client: &'a Client,
        channel_id: Id<ChannelMarker>,
        message_id: Id<MessageMarker>,
    ) -> Self {
        Self {
            client,
            channel_id,
            fields: ChannelMessageUpdateFields {
                content: None,
                embeds: None,
            },
            message_id,
        }
    }

    pub fn content(mut self, content: &'a str) -> Result<Self, MessageValidationError> {
        message::validate_content_length(content)?;
        self.fields.content.replace(content);

        Ok(self)
    }

    pub fn embeds(mut self, embeds: Vec<ChatEmbed>) -> Self {
        self.fields.embeds.replace(embeds);
        self
    }

    pub fn finish(self) -> ResponseFuture<ChatMessage> {
        let client = self.client;

        match self.try_into() {
            Ok(request) => client.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryInto<Request> for ChannelMessageUpdate {
    type Error = Error;

    fn try_into(self) -> Result<Request, Self::Error> {
        let mut request = Request::builder(&Route::ChannelMessageUpdate {
            channel_id: &self.channel_id.value().unwrap_alphanumeric(),
            message_id: &self.message_id.value().unwrap_alphanumeric(),
        });
        request = request.json(&self.fields)?;

        Ok(request.build())
    }
}

#[derive(Serialize)]
struct ChannelMessageUpdateFields<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    embeds: Option<Vec<ChatEmbed>>,
}
