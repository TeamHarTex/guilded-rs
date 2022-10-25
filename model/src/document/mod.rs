//! The document object.

use serde::{Deserialize, Serialize};

use crate::channel::mentions::Mentions;
use crate::datetime::Timestamp;
use crate::id::{
    marker::{ChannelMarker, DocumentMarker, ServerMarker, UserMarker},
    Id,
};

/// Represents a forum thread.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    pub channel_id: Id<ChannelMarker>,
    pub content: String,
    pub created_at: Timestamp,
    pub created_by: Id<UserMarker>,
    pub id: Id<DocumentMarker>,
    pub mentions: Option<Mentions>,
    pub server_id: Id<ServerMarker>,
    pub title: String,
    pub updated_at: Option<Timestamp>,
    pub updated_by: Option<Id<UserMarker>>,
}
