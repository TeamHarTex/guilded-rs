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
pub struct ChannelMessageCreate<'a> {
    client: &'a Client,
    channel_id: Id<ChannelMarker>,
    fields: ChannelMessageCreateFields<'a>,
}

impl<'a> ChannelMessageCreate<'a> {
    pub(crate) fn new(client: &'a Client, channel_id: Id<ChannelMarker>) -> Self {
        Self {
            client,
            channel_id,
            fields: ChannelMessageCreateFields {
                content: None,
                embeds: None,
                is_private: None,
                is_silent: None,
                reply_message_ids: None,
            },
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

    pub fn is_private(mut self, is_private: bool) -> Self {
        self.fields.is_private.replace(is_private);
        self
    }

    pub fn is_silent(mut self, is_silent: bool) -> Self {
        self.fields.is_silent.replace(is_silent);
        self
    }

    pub fn reply_message_ids(mut self, reply_message_ids: Vec<Id<MessageMarker>>) -> Self {
        self.fields.reply_message_ids.replace(reply_message_ids);
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

impl TryInto<Request> for ChannelMessageCreate<'_> {
    type Error = Error;

    fn try_into(self) -> Result<Request, Self::Error> {
        let mut request = Request::builder(&Route::ChannelMessageCreate {
            channel_id: &self.channel_id.value().unwrap_alphanumeric(),
        });
        request = request.json(&self.fields)?;

        Ok(request.build())
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ChannelMessageCreateFields<'a> {
    #[serde::skip_serializing_if = "Option::is_none"]
    content: Option<&'a str>,
    #[serde::skip_serializing_if = "Option::is_none"]
    embeds: Option<Vec<ChatEmbed>>,
    #[serde::skip_serializing_if = "Option::is_none"]
    is_private: Option<bool>,
    #[serde::skip_serializing_if = "Option::is_none"]
    is_silent: Option<bool>,
    #[serde::skip_serializing_if = "Option::is_none"]
    reply_message_ids: Option<Vec<Id<MessageMarker>>>,
}
