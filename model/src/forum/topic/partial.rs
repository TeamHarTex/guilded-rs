//! The partial forum topic object.

use serde::{Deserialize, Serialize};

use crate::datetime::Timestamp;
use crate::id::{
    marker::{ChannelMarker, ForumTopicMarker, ServerMarker, UserMarker, WebhookMarker},
    Id,
};

/// Represents a partial forum topic.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PartialForumTopic {
    pub bumped_at: Option<Timestamp>,
    pub channel_id: Id<ChannelMarker>,
    pub created_at: Timestamp,
    pub created_by: Id<UserMarker>,
    pub created_by_webhook_id: Option<Id<WebhookMarker>>,
    pub id: Id<ForumTopicMarker>,
    pub is_locked: Option<bool>,
    pub is_pinned: Option<bool>,
    pub server_id: Id<ServerMarker>,
    pub title: Option<String>,
    pub updated_at: Option<Timestamp>,
}
