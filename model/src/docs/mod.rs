//! The document object.

use crate::{
    datetime::Timestamp,
    id::{
        marker::{ChannelMarker, DocumentMarker, ServerMarker, UserMarker},
        Id,
    },
};

/// Represents a forum thread.
#[derive(Clone, Debug)]
pub struct Document {
    channel_id: Id<ChannelMarker>,
    content: String,
    created_at: Timestamp,
    created_by: Id<UserMarker>,
    id: Id<DocumentMarker>,
    server_id: Id<ServerMarker>,
    title: String,
    updated_at: Option<Timestamp>,
    updated_by: Option<Id<UserMarker>>,
}

impl Document {
    pub fn channel_id(&self) -> Id<ChannelMarker> {
        self.channel_id.clone()
    }

    pub fn content(&self) -> String {
        self.content.clone()
    }

    pub fn created_at(&self) -> Timestamp {
        self.created_at
    }

    pub fn created_by(&self) -> Id<UserMarker> {
        self.created_by.clone()
    }

    pub fn id(&self) -> Id<DocumentMarker> {
        self.id.clone()
    }

    pub fn server_id(&self) -> Id<ServerMarker> {
        self.server_id.clone()
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn updated_at(&self) -> Option<Timestamp> {
        self.updated_at
    }

    pub fn updated_by(&self) -> Option<Id<UserMarker>> {
        self.updated_by.clone()
    }
}
