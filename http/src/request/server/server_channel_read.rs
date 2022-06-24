use guilded_model::channel::ServerChannel;
use guilded_model::id::{marker::ChannelMarker, Id};

use crate::client::Client;
use crate::error::Error;
use crate::request::Request;
use crate::response::future::ResponseFuture;
use crate::route::Route;

#[must_use = "requests must be configured and executed"]
pub struct ServerChannelRead<'a> {
    client: &'a Client,
    channel_id: Id<ChannelMarker>,
}

impl<'a> ServerChannelRead<'a> {
    pub(crate) fn new(client: &'a Client, channel_id: Id<ChannelMarker>) -> Self {
        Self { client, channel_id }
    }

    pub fn finish(self) -> ResponseFuture<ServerChannel> {
        let client = self.client;

        match self.try_into() {
            Ok(request) => client.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryInto<Request> for ServerChannelRead<'_> {
    type Error = Error;

    fn try_into(self) -> Result<Request, Self::Error> {
        Ok(Request::builder(&Route::ServerChannelRead {
            channel_id: &self.channel_id.value().unwrap_alphanumeric(),
        })
        .build())
    }
}
