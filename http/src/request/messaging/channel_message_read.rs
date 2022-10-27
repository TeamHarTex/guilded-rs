use guilded_model::id::{
    marker::{ChannelMarker, MessageMarker},
    Id,
};
use guilded_model::messaging::ChatMessage;

use crate::client::Client;
use crate::error::Error;
use crate::request::Request;
use crate::response::future::ResponseFuture;
use crate::route::Route;

#[must_use = "requests must be configured and executed"]
pub struct ChannelMessageRead<'a> {
    channel_id: Id<ChannelMarker>,
    client: &'a Client,
    message_id: Id<MessageMarker>,
}

impl<'a> ChannelMessageRead<'a> {
    pub(crate) fn new(
        client: &'a Client,
        channel_id: Id<ChannelMarker>,
        message_id: Id<MessageMarker>,
    ) -> Self {
        Self {
            client,
            channel_id,
            message_id,
        }
    }

    pub fn finish(self) -> ResponseFuture<ChatMessage> {
        let client = self.client;

        match self.try_into() {
            Ok(request) => client.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryInto<Request> for ChannelMessageRead<'_> {
    type Error = Error;

    fn try_into(self) -> Result<Request, Self::Error> {
        Ok(Request::builder(&Route::ChannelMessageRead {
            channel_id: &self.channel_id.value().unwrap_alphanumeric(),
            message_id: &self.message_id.value().unwrap_alphanumeric(),
        })
        .build())
    }
}
