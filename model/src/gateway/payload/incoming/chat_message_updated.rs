//! The ChatMessageUpdated event.

use serde::{Deserialize, Serialize};

use crate::id::{marker::ServerMarker, Id};
use crate::messaging::ChatMessage;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessageUpdated {
    pub message: ChatMessage,
    pub server_id: Id<ServerMarker>,
}
