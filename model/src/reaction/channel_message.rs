use serde::{Deserialize, Serialize};

use crate::datetime::Timestamp;
use crate::id::{
    marker::{ChannelMarker, EmoteMarker, MessageMarker, UserMarker},
    Id,
};

/// Represents a channel message reaction object.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelMessageReaction {
    pub channel_id: Id<ChannelMarker>,
    pub created_by: Id<UserMarker>,
    pub emote_id: Id<EmoteMarker>,
    pub message_id: Id<MessageMarker>,
}
