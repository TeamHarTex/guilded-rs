//! The forum topic object.

use serde::{Deserialize, Serialize};

use crate::channel::mentions::Mentions;
use crate::datetime::Timestamp;
use crate::id::{
    marker::{ChannelMarker, ForumTopicMarker, ServerMarker, UserMarker, WebhookMarker},
    Id,
};

pub mod partial;

/// Represents a forum topic.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ForumTopic {
    pub bumped_at: Option<Timestamp>,
    pub channel_id: Id<ChannelMarker>,
    pub content: Option<String>,
    pub created_at: Timestamp,
    pub created_by: Id<UserMarker>,
    pub created_by_webhook_id: Option<Id<WebhookMarker>>,
    pub id: Id<ForumTopicMarker>,
    pub is_locked: Option<bool>,
    pub is_pinned: Option<bool>,
    pub mentions: Option<Mentions>,
    pub server_id: Id<ServerMarker>,
    pub title: Option<String>,
    pub updated_at: Option<Timestamp>,
}
