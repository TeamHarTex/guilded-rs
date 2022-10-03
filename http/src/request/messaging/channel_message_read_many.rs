use crate::client::Client;
use guilded_model::id::{marker::ChannelMarker, Id};

#[must_use = "requests must be configured and executed"]
pub struct ChannelMessageReadMany<'a> {
    client: &'a Client,
    channel_id: Id<ChannelMarker>,
}

impl<'a> ChannelMessageReadMany<'a> {
    pub fn new(client: &'a Client, channel_id: Id<ChannelMarker>) -> Self {
        Self { client, channel_id }
    }
}
