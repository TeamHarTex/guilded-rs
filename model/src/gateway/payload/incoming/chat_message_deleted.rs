//! The ChatMessageDeleted event.

use serde::{Deserialize, Serialize};

use crate::id::{marker::ServerMarker, Id};
use crate::messaging::deleted_message::DeletedChatMessage;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessageDeleted {
    message: DeletedChatMessage,
    server_id: Id<ServerMarker>,
}

impl ChatMessageDeleted {
    pub fn message(&self) -> DeletedChatMessage {
        self.message.clone()
    }

    pub fn server_id(&self) -> Id<ServerMarker> {
        self.server_id.clone()
    }
}
