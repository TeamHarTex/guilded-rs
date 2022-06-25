use guilded_model::id::{
    marker::{ChannelMarker, MessageMarker},
    Id,
};
use guilded_model::messaging::embed::ChatEmbed;
use serde::Serialize;

use crate::client::Client;

#[must_use = "requests must be configured and executed"]
pub struct ChannelMessageCreate<'a> {
    client: &'a Client,
    channel_id: Id<ChannelMarker>,
    fields: ChannelMessageCreateFields<'a>,
}

impl<'a> ChannelMessageCreate<'a> {
    pub(crate) fn new(client: &'a Client, channel_id: Id<ChannelMarker>) -> Self {
        todo!()
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ChannelMessageCreateFields<'a> {
    content: &'a str,
    #[serde::skip_serializing_if = "Option::is_none"]
    embeds: Option<Vec<ChatEmbed>>,
    #[serde::skip_serializing_if = "Option::is_none"]
    is_private: Option<bool>,
    #[serde::skip_serializing_if = "Option::is_none"]
    is_silent: Option<bool>,
    #[serde::skip_serializing_if = "Option::is_none"]
    reply_message_ids: Option<Vec<Id<MessageMarker>>>,
}
