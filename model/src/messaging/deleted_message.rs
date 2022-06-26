//! Deleted messages.

use serde::{Deserialize, Serialize};

use crate::datetime::Timestamp;
use crate::id::{
    marker::{ChannelMarker, MessageMarker, ServerMarker},
    Id,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeletedChatMessage {
    pub channel_id: Id<ChannelMarker>,
    pub deleted_at: Timestamp,
    pub id: Id<MessageMarker>,
    pub is_private: bool,
    pub server_id: Id<ServerMarker>,
}
