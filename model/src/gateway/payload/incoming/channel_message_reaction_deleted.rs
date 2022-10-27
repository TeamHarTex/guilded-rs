//! The ChannelMessageReactionDeleted event.

use serde::{Deserialize, Serialize};

use crate::id::{marker::ServerMarker, Id};
use crate::reaction::channel_message::ChannelMessageReaction;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelMessageReactionDeleted {
    pub reaction: ChannelMessageReaction,
    pub server_id: Id<ServerMarker>,
}
