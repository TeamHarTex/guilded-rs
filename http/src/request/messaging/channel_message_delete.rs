use guilded_model::id::{
    marker::{ChannelMarker, MessageMarker},
    Id,
};
use crate::client::Client;
use crate::error::Error;
use crate::request::Request;
use crate::response::future::ResponseFuture;
use crate::route::Route;

#[must_use = "requests must be configured and executed"]
pub struct ChannelMessageDelete<'a> {
    client: &'a Client,
    channel_id: Id<ChannelMarker>,
    message_id: Id<MessageMarker>,
}

impl<'a> ChannelMessageDelete<'a> {
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

    pub fn finish(self) -> ResponseFuture<()> {
        let client = self.client;

        match self.try_into() {
            Ok(request) => client.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryInto<Request> for ChannelMessageDelete {
    type Error = Error;

    fn try_into(self) -> Result<Request, Self::Error> {
        let mut request = Request::builder(&Route::ChannelMessageDelete {
            channel_id: &self.channel_id.value().unwrap_alphanumeric(),
            message_id: &self.message_id.value().unwrap_alphanumeric(),
        });

        Ok(request.build())
    }
}
