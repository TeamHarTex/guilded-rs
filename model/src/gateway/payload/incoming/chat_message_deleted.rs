//! The ChatMessageDeleted event.

use serde::{Deserialize, Serialize};

use crate::id::{marker::ServerMarker, Id};
use crate::messaging::deleted_message::DeletedChatMessage;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessageDeleted {
    pub message: DeletedChatMessage,
    pub server_id: Id<ServerMarker>,
}
