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
    channel_id: Id<ChannelMarker>,
    content: Option<String>,
    created_at: Timestamp,
    created_by: Id<UserMarker>,
    created_by_webhook_id: Option<Id<WebhookMarker>>,
    id: Id<ForumThreadMarker>,
    server_id: Id<ServerMarker>,
    title: Option<String>,
    updated_at: Option<Timestamp>,
}

impl ForumTopic {
    pub fn channel_id(&self) -> Id<ChannelMarker> {
        self.channel_id.clone()
    }

    pub fn content(&self) -> Option<String> {
        self.content.clone()
    }

    pub fn created_at(&self) -> Timestamp {
        self.created_at
    }

    pub fn created_by(&self) -> Id<UserMarker> {
        self.created_by.clone()
    }

    pub fn created_by_webhook_id(&self) -> Option<Id<WebhookMarker>> {
        self.created_by_webhook_id.clone()
    }

    pub fn id(&self) -> Id<ForumThreadMarker> {
        self.id.clone()
    }

    pub fn server_id(&self) -> Id<ServerMarker> {
        self.server_id.clone()
    }

    pub fn title(&self) -> Option<String> {
        self.title.clone()
    }

    pub fn updated_at(&self) -> Option<Timestamp> {
        self.updated_at
    }
}
