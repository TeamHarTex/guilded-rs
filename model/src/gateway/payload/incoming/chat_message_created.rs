//! The ChatMessageCreated event.

use serde::{Deserialize, Serialize};

use crate::id::{marker::ServerMarker, Id};
use crate::messaging::ChatMessage;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessageCreated {
    message: ChatMessage,
    server_id: Id<ServerMarker>,
}

impl ChatMessageCreated {
    pub fn message(&self) -> ChatMessage {
        self.message.clone()
    }

    pub fn server_id(&self) -> Id<ServerMarker> {
        self.server_id.clone()
    }
}
