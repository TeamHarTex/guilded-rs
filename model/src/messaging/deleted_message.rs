//! Deleted messages.

use crate::datetime::Timestamp;
use crate::id::{
    marker::{ChannelMarker, MessageMarker, ServerMarker},
    Id,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeletedChatMessage {
    channel_id: Id<ChannelMarker>,
    deleted_at: Timestamp,
    id: Id<MessageMarker>,
    is_private: bool,
    server_id: Id<ServerMarker>,
}

impl DeletedChatMessage {
    pub fn channel_id(&self) -> Id<ChannelMarker> {
        self.channel_id.clone()
    }

    pub fn deleted_at(&self) -> Timestamp {
        self.deleted_at
    }

    pub fn id(&self) -> Id<MessageMarker> {
        self.id.clone()
    }

    pub fn is_private(&self) -> bool {
        self.is_private
    }

    pub fn server_id(&self) -> Id<ServerMarker> {
        self.server_id.clone()
    }
}
