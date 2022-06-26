//! The forum topic object.

use serde::{Deserialize, Serialize};

use crate::datetime::Timestamp;
use crate::id::{
    marker::{ChannelMarker, ForumThreadMarker, ServerMarker, UserMarker, WebhookMarker},
    Id,
};

/// Represents a forum topic.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ForumTopic {
    pub channel_id: Id<ChannelMarker>,
    pub content: Option<String>,
    pub created_at: Timestamp,
    pub created_by: Id<UserMarker>,
    pub created_by_webhook_id: Option<Id<WebhookMarker>>,
    pub id: Id<ForumThreadMarker>,
    pub server_id: Id<ServerMarker>,
    pub title: Option<String>,
    pub updated_at: Option<Timestamp>,
}
