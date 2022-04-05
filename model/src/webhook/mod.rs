//! The webhook object.

use serde::{Deserialize, Serialize};

use crate::datetime::Timestamp;
use crate::id::{marker::{ChannelMarker, ServerMarker, UserMarker, WebhookMarker}, Id};

/// Represents a webhook object.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Webhook {
    channel_id: Id<ChannelMarker>,
    created_at: Timestamp,
    created_by: Id<UserMarker>,
    deleted_at: Option<Timestamp>,
    id: Id<WebhookMarker>,
    name: String,
    server_id: Id<ServerMarker>,
    token: Option<String>,
}

impl Webhook {
    pub fn channel_id(&self) -> Id<ChannelMarker> {
        self.channel_id.clone()
    }

    pub fn created_at(&self) -> Timestamp {
        self.created_at
    }

    pub fn created_by(&self) -> Id<UserMarker> {
        self.created_by.clone()
    }

    pub fn deleted_at(&self) -> Option<Timestamp> {
        self.deleted_at
    }

    pub fn id(&self) -> Id<WebhookMarker> {
        self.id.clone()
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn server_id(&self) -> Id<ServerMarker> {
        self.server_id.clone()
    }

    pub fn token(&self) -> Option<&str> {
        self.token.as_deref()
    }
}
