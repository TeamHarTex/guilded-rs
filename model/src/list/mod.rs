//! The list item object.

use crate::{
    datetime::Timestamp,
    id::{
        marker::{ChannelMarker, ListItemMarker, ServerMarker, UserMarker, WebhookMarker},
        Id,
    },
};

/// Represents a list item.
#[derive(Clone, Debug)]
pub struct ListItem {
    channel_id: Id<ChannelMarker>,
    created_at: Timestamp,
    created_by: Id<UserMarker>,
    created_by_webhook_id: Option<Id<WebhookMarker>>,
    id: Id<ListItemMarker>,
    message: String,
    note: Option<String>,
    server_id: Id<ServerMarker>,
    updated_at: Option<Timestamp>,
    updated_by: Option<Id<UserMarker>>,
}

impl ListItem {
    pub fn channel_id(&self) -> Id<ChannelMarker> {
        self.channel_id.clone()
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

    pub fn id(&self) -> Id<ListItemMarker> {
        self.id.clone()
    }

    pub fn message(&self) -> String {
        self.message.clone()
    }

    pub fn note(&self) -> Option<String> {
        self.note.clone()
    }

    pub fn server_id(&self) -> Id<ServerMarker> {
        self.server_id.clone()
    }

    pub fn updated_at(&self) -> Option<Timestamp> {
        self.updated_at
    }

    pub fn updated_by(&self) -> Option<Id<UserMarker>> {
        self.updated_by.clone()
    }
}
